use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ctx::WG_CTX;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use esp_idf_svc::sys::esp;
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::WgConfig;

pub mod ctx;

use esp_idf_svc::sys::esp_netif_tcpip_exec;
use esp_idf_svc::sys::wg::{
    esp_wireguard_connect,
    esp_wireguard_disconnect,
    esp_wireguard_init,
    esp_wireguard_set_default,
    esp_wireguardif_peer_is_up,
    wireguard_config_t,
    wireguard_ctx_t,
};

/// The maximum number of attempts to sync system time before declaring the call
/// to [sync_systime] a failure.
const MAX_SNTP_ATTEMPTS: u32 = 10;

/// The maximum number of attempts to connect to a wireguard peer before
/// declaring the call to [start_tunnel] a failure.
const MAX_WG_ATTEMPTS: u32 = 10;

/// Syncs system time with UTC using SNTP. This is necessary to establish a
/// wireguard tunnel. Care should thus be taken to always call this function
/// before attempting to establish a connection with a Wireguard peer.
pub fn sync_systime(wifi: Arc<Mutex<EspWifi<'static>>>) -> anyhow::Result<()> {
    let wifi = wifi.lock().unwrap();

    if !wifi.is_connected()? {
        return Err(anyhow::anyhow!("Trying to sync time whilst wifi is down!"));
    }

    let sntp = EspSntp::new_default()?;

    for retries in 0..=MAX_SNTP_ATTEMPTS {
        if sntp.get_sync_status() == SyncStatus::Completed {
            log::info!("Time synchronized successfully.");
            break;
        }
        log::info!("Waiting for time synchronization...");
        std::thread::park_timeout(std::time::Duration::from_secs(1));

        if retries == MAX_SNTP_ATTEMPTS {
            log::error!("Failed to synchronize time! Is internet available?");
            return Err(anyhow::anyhow!("Failed to synchronize time!"));
        }
    }

    Ok(())
}

/// Establishes a tunnel with the peer defined in the `nvs` configuration.
///
/// This configuration should be set using the [`WgConfig`] struct. Care should
/// be taken to always call this function with the `STA netif` connected to an
/// access point.
///
/// No internet connection will result in the function returning
/// after [`MAX_SNTP_ATTEMPTS`] to sync system time have been expanded. An
/// invalid configuration will result in a cleanup of all allocated ressources
/// after [`MAX_WG_ATTEMPTS`] have been expanded.
///
/// This function sets the [`WG_CTX`] global variable. Care should be taken
/// NEVER TO DROP this context as it would unvariably result in undefined
/// behavior or crash the program.
pub fn start_tunnel(nvs: Arc<Mutex<EspNvs<NvsDefault>>>) -> anyhow::Result<()> {
    let wg_conf = WgConfig::get_config(nvs)?;

    unsafe {
        let config = &mut wireguard_config_t {
            private_key: CString::new(wg_conf.client_private_key.as_str())?.into_raw(),
            listen_port: 51820,
            fw_mark: 0,
            public_key: CString::new(wg_conf.server_public_key.as_str())?.into_raw(),
            preshared_key: core::ptr::null_mut(),
            allowed_ip: CString::new("0.0.0.0")?.into_raw(),
            allowed_ip_mask: CString::new("0.0.0.0")?.into_raw(),
            endpoint: CString::new(wg_conf.address.as_str())?.into_raw(),
            port: wg_conf.port.as_str().parse()?,
            persistent_keepalive: 20,
        } as *mut _;

        let ctx = &mut wireguard_ctx_t {
            config,
            netif: core::ptr::null_mut(),
            netif_default: core::ptr::null_mut(),
        } as *mut _;

        log::info!("Initializing wireguard..");

        esp!(esp_wireguard_init(config, ctx))?;

        log::info!("Connecting to peer..");

        // Everything to do with ip shenanigans has to be executed in a tcpip context.
        esp!(esp_netif_tcpip_exec(Some(wg_connect_wrapper), ctx as *mut core::ffi::c_void))?;

        for i in 0..=MAX_WG_ATTEMPTS {
            if i == MAX_WG_ATTEMPTS {
                log::error!("Max retries reached, cleaning up.");

                // While we're not connected yet, this allows us to fail gracefully by
                // deinitializing the entire stack to start from a clean slate the
                // next time we make an attempt to connect to a peer.
                esp!(esp_netif_tcpip_exec(Some(wg_disconnect_wrapper), ctx as *mut core::ffi::c_void))?;

                return Err(anyhow::anyhow!("Failed to connect to peer, cleaning up."));
            }

            match esp!(esp_wireguardif_peer_is_up(ctx)) {
                Ok(_) => {
                    log::info!("Peer is up!");
                    break;
                }
                Err(_) => {
                    log::warn!("Peer is down..");
                    std::thread::park_timeout(Duration::from_millis(1000));
                }
            }
        }

        log::info!("Setting default gateway..");

        esp!(esp_netif_tcpip_exec(
            Some(wg_set_default_wrapper),
            ctx as *mut core::ffi::c_void
        ))?;

        // Keep ctx in scope with global context.
        let mut guard = WG_CTX.lock().unwrap();
        guard.set(ctx);

        Ok(())
    }
}

/// Wrapper for a C function that requires execution in a tcpip context using
/// the [`esp_netif_tcpip_exec`] utility.
unsafe extern "C" fn wg_set_default_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_set_default(ctx as *mut wireguard_ctx_t)
}

/// Wrapper for a C function that requires execution in a tcpip context using
/// the [`esp_netif_tcpip_exec`] utility.
unsafe extern "C" fn wg_connect_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_connect(ctx as *mut wireguard_ctx_t)
}

/// Wrapper for a C function that requires execution in a tcpip context using
/// the [`esp_netif_tcpip_exec`] utility.
unsafe extern "C" fn wg_disconnect_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_disconnect(ctx as *mut wireguard_ctx_t)
}

/// Ends an established tunnel with the peer defined in the `nvs` configuration.
///
/// This function resets the [`WG_CTX`] global variable. Care should be taken
/// NEVER TO DROP this context before the execution of this function as it would
/// unvariably result in either undefined behavior or crash the program.  
pub fn end_tunnel() -> anyhow::Result<()> {
    log::info!("Disconnecting from peer..");

    let mut guard = WG_CTX.lock().unwrap();

    if !guard.is_set() {
        log::error!("Called end_wg_tunnel with null context!");
        return Err(anyhow::anyhow!("Null ctx pointer"));
    }

    unsafe {
        esp!(esp_netif_tcpip_exec(
            Some(wg_disconnect_wrapper),
            guard.0 as *mut core::ffi::c_void
        ))?;
    }

    guard.reset();

    Ok(())
}
