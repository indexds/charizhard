use crate::utils::heapless::HeaplessString;
use crate::utils::nvs::{NvsKeys, NvsWifi};
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, EspWifi};
use log::info;
use std::sync::{Arc, Mutex};

pub fn start_ap(wifi: Arc<Mutex<BlockingWifi<EspWifi<'static>>>>) -> anyhow::Result<()> {
    let mut wifi = wifi
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock Wifi Mutex."))?;

    let mut ap_ssid = HeaplessString::<32>::new();
    let mut ap_passwd = HeaplessString::<64>::new();
    ap_ssid.push_str("charizhard")?;
    ap_passwd.push_str("testpassword")?;

    let ap_config = AccessPointConfiguration {
        ssid: ap_ssid.try_into()?,
        password: ap_passwd.try_into()?,
        auth_method: AuthMethod::WPA2Personal,
        ssid_hidden: false,
        channel: 1,
        ..Default::default()
    };

    let dummy_sta_config = ClientConfiguration {
        ..Default::default()
    };

    wifi.set_configuration(&Configuration::Mixed(dummy_sta_config, ap_config))?;

    wifi.start()?;
    info!("WIFI STARTED..");

    Ok(())
}

pub fn connect_wifi(
    wifi: &Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
    nvs: &Arc<Mutex<EspNvs<NvsDefault>>>,
) -> anyhow::Result<()> {
    let nvs = nvs
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;
    let mut wifi = wifi
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock WIFI Mutex."))?;

    if wifi.is_connected()? {
        wifi.disconnect()?;
    }

    let ssid = NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_SSID)?.inner();
    let password = NvsWifi::get_field::<64>(&nvs, NvsKeys::STA_PASSWD)?.inner();

    info!("ssid:{}", ssid);
    info!("password:{}", password);

    let sta_config = if password.trim().is_empty() {
        ClientConfiguration {
            ssid,
            password,
            ..Default::default()
        }
    } else {
        ClientConfiguration {
            ssid,
            auth_method: AuthMethod::None,
            ..Default::default()
        }
    };

    //TEMPORARY! TO BE DELETED ONCE THE ETHERNET BRIDGE IS UP--------------
    let mut ap_ssid = HeaplessString::<32>::new();
    let mut ap_passwd = HeaplessString::<64>::new();
    ap_ssid.push_str("charizhard")?;
    ap_passwd.push_str("testpassword")?;

    let ap_config = AccessPointConfiguration {
        ssid: ap_ssid.try_into()?,
        password: ap_passwd.try_into()?,
        auth_method: AuthMethod::WPA2Personal,
        ssid_hidden: false,
        channel: 1,
        ..Default::default()
    };
    wifi.set_configuration(&Configuration::Mixed(sta_config, ap_config))?;
    //END TEMPORARY--------------------------------------------------------

    // wifi.set_configuration(&Configuration::Client(sta_configuration))?;
    wifi.connect()?;

    Ok(())
}

pub fn disconnect_wifi(wifi: &Arc<Mutex<BlockingWifi<EspWifi<'static>>>>) -> anyhow::Result<()> {
    let mut wifi = wifi
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock WIFI Mutex."))?;

    if !wifi.is_started()? {
        return Ok(());
    }

    if wifi.is_connected()? {
        wifi.disconnect()?;
    }

    Ok(())
}
