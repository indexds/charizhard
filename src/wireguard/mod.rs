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

const MAX_SNTP_RETRIES: u32 = 10;
const MAX_WG_RETRIES: u32 = 10;

// Wireguard needs time to be synced to UTC to not explode
pub fn sync_sntp(wifi: Arc<Mutex<EspWifi<'static>>>) -> anyhow::Result<()> {
    let wifi = wifi.lock().unwrap();

    if !wifi.is_connected()? {
        return Err(anyhow::anyhow!("Trying to sync time whilst wifi is down!"));
    }

    let sntp = EspSntp::new_default()?;

    for retries in 0..=MAX_SNTP_RETRIES {
        if sntp.get_sync_status() == SyncStatus::Completed {
            log::info!("Time synchronized successfully.");
            break;
        }
        log::info!("Waiting for time synchronization...");
        std::thread::park_timeout(std::time::Duration::from_secs(1));

        if retries == MAX_SNTP_RETRIES {
            log::error!("Failed to synchronize time! Is internet available?");
            return Err(anyhow::anyhow!("Failed to synchronize time!"));
        }
    }

    Ok(())
}

pub fn start_wg_tunnel(nvs: Arc<Mutex<EspNvs<NvsDefault>>>) -> anyhow::Result<()> {
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

        // Everything to do with ip shenanigans has to be executed in a tcpip context
        esp!(esp_netif_tcpip_exec(Some(wg_connect_wrapper), ctx as *mut core::ffi::c_void))?;

        for i in 0..=MAX_WG_RETRIES {
            if i == MAX_WG_RETRIES {
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

        let mut global_ctx = WG_CTX.lock().unwrap();
        *global_ctx = Some(ctx::WireguardCtx::new(ctx));

        Ok(())
    }
}

pub unsafe extern "C" fn wg_set_default_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_set_default(ctx as *mut wireguard_ctx_t)
}

pub unsafe extern "C" fn wg_connect_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_connect(ctx as *mut wireguard_ctx_t)
}

pub unsafe extern "C" fn wg_disconnect_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_disconnect(ctx as *mut wireguard_ctx_t)
}

pub fn end_wg_tunnel() -> anyhow::Result<()> {
    let mut global_ctx = WG_CTX.lock().unwrap();

    if global_ctx.is_none() {
        log::error!("Failed to disconnect from peer! Were we connected in the first place?");
        return Err(anyhow::anyhow!("Failed to disconnect from peer!"));
    }

    let ctx = global_ctx.as_mut().unwrap().0;

    unsafe {
        log::info!("Disconnecting from peer..");

        esp!(esp_netif_tcpip_exec(Some(wg_disconnect_wrapper), ctx as *mut core::ffi::c_void))?;

        log::info!("Resetting global context..");

        *global_ctx = None;
    }

    Ok(())
}
