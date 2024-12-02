use crate::utils::nvs::NvsWireguard;
use esp_idf_hal::sys::{ESP_FAIL, ESP_OK};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use std::ffi::CString;
use std::sync::{Arc, Mutex};

use esp_idf_svc::sys::EspError;

#[allow(unused_imports)]
use esp_idf_svc::sys::wg::{
    esp_wireguard_connect,
    esp_wireguard_disconnect,
    esp_wireguard_init,
    esp_wireguard_set_default,
    esp_wireguardif_peer_is_up,
    wireguard_config_t,
    wireguard_ctx_t,
};

use esp_idf_svc::sys::esp_netif_tcpip_exec;

#[allow(unused_must_use)]
pub fn start_wg_tunnel(
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
) -> anyhow::Result<*mut wireguard_ctx_t> {
    let nvs = nvs
        .try_lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex!"))?;

    let nvs_wg = NvsWireguard::new(&nvs)?;

    let endpoint = CString::new(nvs_wg.wg_addr.clean_string().as_str())?.into_raw();
    let port: i32 = nvs_wg.wg_port.clean_string().as_str().parse()?;

    let private_key = CString::new(nvs_wg.wg_client_priv_key.clean_string().as_str())?.into_raw();
    let public_key = CString::new(nvs_wg.wg_server_pub_key.clean_string().as_str())?.into_raw();

    let allowed_ip = CString::new("0.0.0.0")?.into_raw();
    let allowed_ip_mask = CString::new("0.0.0.0")?.into_raw();

    let sntp = EspSntp::new_default()?;

    for retries in 0..=10 {
        if sntp.get_sync_status() == SyncStatus::Completed {
            log::info!("Time synchronized successfully.");
            break;
        }
        log::info!("Waiting for time synchronization...");
        std::thread::park_timeout(std::time::Duration::from_secs(1));

        if retries == 10 {
            log::error!("Failed to synchronize time! Is internet available?");
            return Err(anyhow::anyhow!("Failed to synchronize time!"));
        }
    }

    unsafe {
        let mut wg_config_t = wireguard_config_t {
            private_key,
            listen_port: 51820,
            fw_mark: 0,
            public_key,
            preshared_key: core::ptr::null_mut(),
            allowed_ip,
            allowed_ip_mask,
            endpoint,
            port,
            persistent_keepalive: 20,
        };

        let config_ptr = &mut wg_config_t as *mut _;

        let mut wg_ctx_t = wireguard_ctx_t {
            config: config_ptr,
            netif: core::ptr::null_mut(),
            netif_default: core::ptr::null_mut(),
        };

        let ctx_ptr = &mut wg_ctx_t as *mut _;

        let res = esp_wireguard_init(config_ptr, ctx_ptr);
        if res != ESP_OK {
            log::error!("Failed to initialize WireGuard! - CODE: {}", res);
            return Err(EspError::from(res).unwrap().into());
        } else {
            log::info!("WireGuard initialized successfully.");
        }        

        let res = esp_netif_tcpip_exec(Some(wg_connect_wrapper), ctx_ptr as *mut core::ffi::c_void);
        if res != ESP_OK {
            log::error!("Failed to connect to wireguard peer! - CODE: {}", res);
            return Err(EspError::from(res).unwrap().into());
        }

        while esp_wireguardif_peer_is_up(ctx_ptr) != ESP_OK {
            log::info!("Peer is down..");
            std::thread::park_timeout(std::time::Duration::from_secs(1));
        }
        log::info!("Peer is up!");

        let res = esp_wireguard_set_default(ctx_ptr);
        if res != ESP_OK {
            log::error!("Failed to set default gateway! CODE: {}", res);
            return Err(EspError::from(res).unwrap().into());
        }

        //Prevent memleak
        CString::from_raw(allowed_ip);
        CString::from_raw(allowed_ip_mask);
        CString::from_raw(endpoint);
        CString::from_raw(private_key);
        CString::from_raw(public_key);

        Ok(ctx_ptr)
    }
}

pub unsafe extern "C" fn wg_connect_wrapper(ctx: *mut core::ffi::c_void) -> i32 {
    if ctx.is_null() {
        log::error!("WireGuard context is null in the callback!");
        return ESP_FAIL;
    }

    let res = esp_wireguard_connect(ctx as *mut wireguard_ctx_t);

    return res;
}

#[allow(dead_code, unused_variables)]
pub fn end_wg_tunnel(
    ctx: *mut wireguard_ctx_t,
) -> anyhow::Result<()>{

    unsafe {
        let res = esp_wireguard_disconnect(ctx);
        if res != ESP_OK {
            log::error!("Failed to set disconnect from peer! CODE: {}", res);
            return Err(EspError::from(res).unwrap().into());
        }

        *(ctx as *mut *mut wireguard_ctx_t) = core::ptr::null_mut();

    }
    Ok(())
}
