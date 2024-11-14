use crate::utils::nvs::NvsWireguard;
use esp_idf_hal::sys::ESP_OK;
use esp_idf_svc::handle::RawHandle;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::sys::wireguard::{
    esp_wireguard_connect,
    esp_wireguard_init,
    // esp_wireguard_disconnect,
    // esp_wireguard_set_default,
    esp_wireguardif_peer_is_up,
    wireguard_config_t,
    wireguard_ctx_t,
};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;
use std::ffi::CString;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};

pub fn start_wg_tunnel(
    nvs: &Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: &Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
) -> anyhow::Result<*mut wireguard_ctx_t> {
    let mut wifi = wifi.lock().map_err(|_| anyhow::anyhow!("Failed to lock Wifi Mutex."))?;
    let nvs = nvs.lock().map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;

    let wg_config = NvsWireguard::new(&nvs)?;

    let endpoint = CString::new(wg_config.wg_addr.clean_string().as_str())?.into_raw();
    let port: i32 = wg_config.wg_port.clean_string().as_str().parse()?;

    let private_key = CString::new(wg_config.wg_client_priv_key.clean_string().as_str())?.into_raw();
    let public_key = CString::new(wg_config.wg_server_pub_key.clean_string().as_str())?.into_raw();
    
    let allowed_ip = CString::new("192.168.72.150")?.into_raw();
    let allowed_ip_mask = CString::new("255.255.255.0")?.into_raw();

    let mut wg_conf = wireguard_config_t {
        endpoint,
        port,
        private_key,
        public_key,
        allowed_ip,
        allowed_ip_mask,
        fw_mark: 0,
        listen_port: 51820,
        persistent_keepalive: 20,
        preshared_key: null_mut(),
    };

    let wg_conf_ptr: *mut wireguard_config_t = &mut wg_conf;

    let wifi_netif_handle = wifi.wifi_mut().sta_netif_mut().handle();

    let mut ctx_t = wireguard_ctx_t {
        config: wg_conf_ptr,
        netif: wifi_netif_handle as *mut _,
        netif_default: null_mut(),
    };

    let ctx_ptr: *mut wireguard_ctx_t = &mut ctx_t;

    unsafe {
        let init_res = esp_wireguard_init(wg_conf_ptr, ctx_ptr);
        if init_res != ESP_OK {
            return Err(anyhow::anyhow!("Failed to initialize WireGuard: {}", init_res));
        }

        let connect_res = esp_wireguard_connect(ctx_ptr);
        if connect_res != ESP_OK {
            return Err(anyhow::anyhow!("Failed to connect WireGuard: {}", connect_res));
        }

        let peer_status = esp_wireguardif_peer_is_up(ctx_ptr);
        if peer_status == ESP_OK {
            info!("WIREGUARD INTERFACE UP");
        } else {
            return Err(anyhow::anyhow!("WireGuard peer is not up: {}", peer_status));
        }
    }

    // Prevent memleak
    #[allow(unused_must_use)]
    unsafe {
        CString::from_raw(private_key);
        CString::from_raw(public_key);
        CString::from_raw(endpoint);
        CString::from_raw(allowed_ip);
        CString::from_raw(allowed_ip_mask);
    }

    Ok(ctx_ptr)
}
