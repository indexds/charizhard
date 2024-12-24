use std::str::FromStr;
use std::sync::{Arc, Mutex};

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi, WifiDriver};

use crate::utils::nvs::{NvsKeys, NvsWifi};

pub fn init_netif(
    modem: Modem,
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
) -> anyhow::Result<Arc<Mutex<EspWifi<'static>>>> {
    log::info!("Installing wifi netif...");

    let wifi_driver = WifiDriver::new(modem, sysloop.clone(), Some(nvs.clone()))?;

    let wifi_netif = EspWifi::wrap_all(
        wifi_driver,
        EspNetif::new_with_conf(&NetifConfiguration {
            stack: NetifStack::Sta,
            ..NetifConfiguration::wifi_default_client()
        })?,
    )?;

    log::info!("Installed wifi netif!");

    Ok(Arc::new(Mutex::new(wifi_netif)))
}

pub fn set_configuration(
    nvs_config: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi_netif: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<()> {
    log::info!("Setting wifi configuration...");

    let mut wifi_netif = wifi_netif.lock().unwrap();
    let nvs = nvs_config.lock().unwrap();

    // TEMP SET
    // NvsWifi::set_field(&mut nvs, NvsKeys::STA_SSID, "fishingrodent")?;
    // NvsWifi::set_field(&mut nvs, NvsKeys::STA_PASSWD, "iliketrains")?;
    // NvsWifi::set_field(&mut nvs, NvsKeys::STA_AUTH_METHOD, "wpa2personal")?;
    // END TEMP SET

    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_SSID)?,
        password: NvsWifi::get_field::<64>(&nvs, NvsKeys::STA_PASSWD)?,
        auth_method: AuthMethod::from_str(NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_AUTH_METHOD)?.as_str())?,
        ..Default::default()
    });

    wifi_netif.set_configuration(&wifi_config)?;

    log::info!("Wifi configuration set!");

    Ok(())
}

pub fn connect(wifi_netif: Arc<Mutex<EspWifi<'static>>>) -> anyhow::Result<()> {
    log::info!("Connecting to access point..");

    let mut wifi = wifi_netif.lock().unwrap();

    if !wifi.is_started()? {
        wifi.start()?;
    }

    if wifi.is_connected()? {
        return Ok(());
    }

    wifi.connect()?;

    Ok(())
}

pub fn disconnect(wifi_netif: Arc<Mutex<EspWifi<'static>>>) -> anyhow::Result<()> {
    log::warn!("Disconnecting from access point..");

    let mut wifi = wifi_netif.lock().unwrap();

    if !wifi.is_started()? {
        wifi.start()?;
    }

    if !wifi.is_connected()? {
        return Ok(());
    }

    wifi.disconnect()?;

    Ok(())
}
