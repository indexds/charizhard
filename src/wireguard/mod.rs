use std::ffi::CString;
use std::sync::{Arc, Mutex};

use ctx::WG_CTX;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use esp_idf_svc::sys::esp;
pub use esp_idf_svc::sys::wg::wireguard_ctx_t;
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::NvsWireguard;

pub mod ctx;

use esp_idf_svc::sys::esp_netif_tcpip_exec;
use esp_idf_svc::sys::wg::{
    esp_wireguard_connect,
    esp_wireguard_disconnect,
    esp_wireguard_init,
    esp_wireguard_set_default,
    esp_wireguardif_peer_is_up,
    wireguard_config_t,
};

const MAX_SNTP_RETRIES: u32 = 10;

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
    let nvs = nvs.lock().unwrap();

    let nvs_wg = NvsWireguard::new(&nvs)?;

    let endpoint = CString::new("34.66.61.218")?.into_raw();
    // let endpoint =
    // CString::new(nvs_wg.wg_addr.clean_string().as_str())?.into_raw();

    unsafe {
        let config = &mut wireguard_config_t {
            private_key: CString::new(nvs_wg.wg_client_priv_key.clean_string().as_str())?.into_raw(),
            listen_port: 51820,
            fw_mark: 0,
            public_key: CString::new(nvs_wg.wg_server_pub_key.clean_string().as_str())?.into_raw(),
            preshared_key: core::ptr::null_mut(),
            allowed_ip: CString::new("0.0.0.0")?.into_raw(),
            allowed_ip_mask: CString::new("0.0.0.0")?.into_raw(),
            endpoint,
            port: nvs_wg.wg_port.clean_string().as_str().parse()?,
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

        esp!(esp_netif_tcpip_exec(Some(wg_connect_wrapper), ctx as *mut core::ffi::c_void))?;

        loop {
            match esp!(esp_wireguardif_peer_is_up(ctx)) {
                Ok(_) => break,
                Err(_) => log::warn!("Peer is down.."),
            }
        }

        log::info!("Peer is up!");

        esp!(esp_netif_tcpip_exec(
            Some(wg_set_default_wrapper),
            ctx as *mut core::ffi::c_void
        ))?;
        log::info!("Default gateway set successfully!");

        let mut global_ctx = WG_CTX.lock().unwrap();
        *global_ctx = Some(crate::wireguard::ctx::WireguardCtx::new(ctx));

        Ok(())
    }
}

pub unsafe extern "C" fn wg_set_default_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_set_default(ctx as *mut wireguard_ctx_t)
}

pub unsafe extern "C" fn wg_connect_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    esp_wireguard_connect(ctx as *mut wireguard_ctx_t)
}

pub fn end_wg_tunnel() -> anyhow::Result<()> {
    let mut global_ctx = WG_CTX.lock().unwrap();

    let ctx = global_ctx.as_ref().unwrap().get_raw();

    unsafe {
        log::info!("Disconnecting from peer..");
        esp!(esp_wireguard_disconnect(ctx))?;
        log::info!("Disconnected.");

        *global_ctx = None;
    }
    Ok(())
}
