use crate::utils::heapless::HeaplessString;
use crate::utils::nvs::{NvsKeys, NvsWifi};
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::{AccessPointConfiguration, BlockingWifi, EspWifi};
use log::info;
use std::sync::{Arc, Mutex};

pub fn start_ap(wifi: &mut Arc<Mutex<BlockingWifi<EspWifi<'static>>>>) -> anyhow::Result<()> {
    let mut wifi = wifi
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock Wifi Mutex."))?;

    let mut ap_ssid = HeaplessString::<32>::new();
    let mut ap_passwd = HeaplessString::<64>::new();
    ap_ssid.push_str("charizhard")?;
    ap_passwd.push_str("testpassword")?;

    let ap_configuration = AccessPointConfiguration {
        ssid: ap_ssid.try_into()?,
        password: ap_passwd.try_into()?,
        ..Default::default()
    };

    wifi.set_configuration(&Configuration::AccessPoint(ap_configuration))?;

    wifi.start()?;
    info!("WIFI STARTED..");

    wifi.wait_netif_up()?;
    info!("WIFI NETIF UP.");

    Ok(())
}

pub fn connect_wifi(
    wifi: &mut Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
) -> anyhow::Result<()> {
    let nvs = nvs
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;
    let mut wifi = wifi
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock WIFI Mutex."))?;
    

    if !wifi.is_started()?{
        wifi.start()?;
    }

    if wifi.is_connected()? {
        wifi.disconnect()?;
    }

    let ssid = NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_SSID)?.inner();
    let password = NvsWifi::get_field::<64>(&nvs, NvsKeys::STA_PASSWD)?.inner();

    let sta_configuration = ClientConfiguration {
        ssid,
        password,
        scan_method: esp_idf_svc::wifi::ScanMethod::FastScan,
        ..Default::default()
    };

    wifi.set_configuration(&Configuration::Client(sta_configuration))?;

    wifi.connect()?;

    Ok(())
}

pub fn disconnect_wifi(
    wifi: &mut Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
) -> anyhow::Result<()> {
    let mut wifi = wifi
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock WIFI Mutex."))?;

    if !wifi.is_started()?{
        return Ok(())
    }

    if wifi.is_connected()? {
        wifi.disconnect()?;
    }

    Ok(())
}
