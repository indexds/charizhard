use crate::utils::nvs::{NvsKeys, NvsWifi};
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, EspWifi};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub fn connect_wifi(
    wifi: &Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
    nvs: &Arc<Mutex<EspNvs<NvsDefault>>>,
) -> anyhow::Result<()> {
    let nvs = nvs.lock().map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;
    let mut wifi = wifi.lock().map_err(|_| anyhow::anyhow!("Failed to lock WIFI Mutex."))?;

    if wifi.is_connected()? {
        wifi.disconnect()?;
    }
    let ssid = NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_SSID)?
        .clean_string()
        .inner();
    let password = NvsWifi::get_field::<64>(&nvs, NvsKeys::STA_PASSWD)?
        .clean_string()
        .inner();
    let auth_method = NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_AUTH_METHOD)?
        .clean_string()
        .inner();

    let sta_config = if password.trim().is_empty() {
        ClientConfiguration {
            ssid,
            auth_method: AuthMethod::None,
            ..Default::default()
        }
    } else {
        ClientConfiguration {
            ssid,
            password,
            auth_method: AuthMethod::from_str(auth_method.as_str())?,
            ..Default::default()
        }
    };

    // TEMPORARY! TO BE DELETED ONCE THE ETHERNET BRIDGE IS UP--------------
    // let mut ap_ssid = HeaplessString::<32>::new();
    // let mut ap_passwd = HeaplessString::<64>::new();
    // ap_ssid.push_str("charizhard")?;
    // ap_passwd.push_str("testpassword")?;

    // let ap_config = AccessPointConfiguration {
    //     ssid: ap_ssid.try_into()?,
    //     password: ap_passwd.try_into()?,
    //     auth_method: AuthMethod::WPA2Personal,
    //     ssid_hidden: false,
    //     channel: 1,
    //     ..Default::default()
    // };
    // wifi.set_configuration(&Configuration::Mixed(sta_config, ap_config))?;
    // END TEMPORARY--------------------------------------------------------

    wifi.set_configuration(&Configuration::Client(sta_config))?;
    wifi.start()?;
    wifi.connect()?;

    Ok(())
}

pub fn disconnect_wifi(wifi: &Arc<Mutex<BlockingWifi<EspWifi<'static>>>>) -> anyhow::Result<()> {
    let mut wifi = wifi.lock().map_err(|_| anyhow::anyhow!("Failed to lock WIFI Mutex."))?;

    if !wifi.is_started()? {
        return Ok(());
    }

    if wifi.is_connected()? {
        wifi.disconnect()?;
    }

    Ok(())
}

// pub fn start_ap(wifi: Arc<Mutex<BlockingWifi<EspWifi<'static>>>>) -> anyhow::Result<()> {
//     let mut wifi = wifi.lock().map_err(|_| anyhow::anyhow!("Failed to lock Wifi Mutex."))?;
//     use crate::utils::heapless::HeaplessString;
//     let mut ap_ssid = HeaplessString::<32>::new();
//     let mut ap_passwd = HeaplessString::<64>::new();
//     ap_ssid.push_str("charizhard")?;
//     ap_passwd.push_str("testpassword")?;

//     let ap_config = AccessPointConfiguration {
//         ssid: ap_ssid.try_into()?,
//         password: ap_passwd.try_into()?,
//         auth_method: AuthMethod::WPA2Personal,
//         ssid_hidden: false,
//         channel: 1,
//         ..Default::default()
//     };

//     let dummy_sta_config = ClientConfiguration {
//         ..Default::default()
//     };

//     wifi.set_configuration(&Configuration::Mixed(dummy_sta_config, ap_config))?;

//     wifi.start()?;
//     info!("WIFI STARTED..");

//     Ok(())
// }
