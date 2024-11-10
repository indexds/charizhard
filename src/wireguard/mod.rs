use crate::utils::nvs::NvsWireguard;
use esp_idf_hal::sys::wireguard::{
    esp_wireguard_connect,
    esp_wireguard_init,
    // esp_wireguard_disconnect,
    // esp_wireguard_set_default,
    esp_wireguardif_peer_is_up,
    wireguard_config_t,
    wireguard_ctx_t,
};
use esp_idf_hal::sys::ESP_OK;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use log::info;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

pub fn start_wg_tunnel(
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
) -> anyhow::Result<Arc<Mutex<*mut wireguard_ctx_t>>> {
    let nvs = nvs
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;
    let wg_config = NvsWireguard::new(&nvs)?;

    let private_key_ptr = CString::new(wg_config.wg_client_priv_key.as_str())?.into_raw();
    let public_key_ptr = CString::new(wg_config.wg_server_pub_key.as_str())?.into_raw();
    let endpoint_ptr = CString::new(wg_config.wg_addr.as_str())?.into_raw();
    let in_out_port: i32 = wg_config.wg_port.as_str().parse()?;

    let mut wg_conf_t = wireguard_config_t {
        private_key: private_key_ptr,
        public_key: public_key_ptr,
        listen_port: in_out_port,
        port: in_out_port,
        endpoint: endpoint_ptr,
        persistent_keepalive: 20,
        ..Default::default()
    };

    let wg_conf_ptr: *mut wireguard_config_t = &mut wg_conf_t;

    let mut ctx_t = wireguard_ctx_t {
        config: wg_conf_ptr,
        ..Default::default()
    };

    let ctx_ptr: *mut wireguard_ctx_t = &mut ctx_t;

    unsafe {
        let init_res = esp_wireguard_init(wg_conf_ptr, ctx_ptr);
        if init_res != ESP_OK {
            return Err(anyhow::anyhow!(
                "Failed to initialize WireGuard: {}",
                init_res
            ));
        }

        let connect_res = esp_wireguard_connect(ctx_ptr);
        if connect_res != ESP_OK {
            return Err(anyhow::anyhow!(
                "Failed to connect WireGuard: {}",
                connect_res
            ));
        }

        let peer_status = esp_wireguardif_peer_is_up(ctx_ptr);
        if peer_status == ESP_OK {
            info!("WIREGUARD INTERFACE UP");
        } else {
            return Err(anyhow::anyhow!("WireGuard peer is not up: {}", peer_status));
        }
    }

    //Prevent memleak
    #[allow(unused_must_use)]
    unsafe {
        CString::from_raw(private_key_ptr);
        CString::from_raw(public_key_ptr);
        CString::from_raw(endpoint_ptr);
    }

    Ok(Arc::new(Mutex::new(ctx_ptr)))
}
