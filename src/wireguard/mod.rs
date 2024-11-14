use crate::utils::nvs::NvsWireguard;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::ffi::CString;
use std::sync::{Arc, Mutex};

pub fn start_wg_tunnel(
    nvs: &Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: &Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
) -> anyhow::Result<()> {
    let wifi = wifi.lock().map_err(|_| anyhow::anyhow!("Failed to lock Wifi Mutex."))?;
    let nvs = nvs.lock().map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;

    let wg_config = NvsWireguard::new(&nvs)?;

    let endpoint = CString::new(wg_config.wg_addr.clean_string().as_str())?.into_raw();
    let port: i32 = wg_config.wg_port.clean_string().as_str().parse()?;

    let private_key = CString::new(wg_config.wg_client_priv_key.clean_string().as_str())?.into_raw();
    let public_key = CString::new(wg_config.wg_server_pub_key.clean_string().as_str())?.into_raw();
    
    let allowed_ip = CString::new("192.168.72.150")?.into_raw();
    let allowed_ip_mask = CString::new("255.255.255.0")?.into_raw();

    //BEGIN WG LOGIC



    //END WG LOGIC

    // Prevent memleak
    #[allow(unused_must_use)]
    unsafe {
        CString::from_raw(private_key);
        CString::from_raw(public_key);
        CString::from_raw(endpoint);
        CString::from_raw(allowed_ip);
        CString::from_raw(allowed_ip_mask);
    }

    Ok(())
}
