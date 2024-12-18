use crate::utils::nvs::{NvsKeys, NvsWifi};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::ipv4::{
    ClientConfiguration as IpClientConfiguration,
    ClientSettings as IpClientSettings,
    Configuration as IpConfiguration,
    Ipv4Addr,
    Mask,
    Subnet,
};
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi, WifiDriver};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub fn init_wifi(
    modem: Modem,
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
) -> anyhow::Result<Arc<Mutex<EspWifi<'static>>>> {
    let wifi_driver = WifiDriver::new(modem, sysloop.clone(), Some(nvs.clone()))?;

    let wifi_netif = EspWifi::wrap_all(
        wifi_driver,
        EspNetif::new_with_conf(&NetifConfiguration {
            ip_configuration: Some(IpConfiguration::Client(IpClientConfiguration::Fixed(IpClientSettings {
                ip: Ipv4Addr::new(192, 168, 71, 200),
                subnet: Subnet {
                    gateway: Ipv4Addr::new(192, 168, 71, 1),
                    mask: Mask(24),
                },
                dns: Some(Ipv4Addr::new(1, 1, 1, 1)),
                secondary_dns: Some(Ipv4Addr::new(1, 0, 0, 1)),
            }))),
            stack: NetifStack::Sta,
            ..NetifConfiguration::wifi_default_client()
        })?,
    )?;

    Ok(Arc::new(Mutex::new(wifi_netif)))
}

pub fn set_configuration(
    nvs_config: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi_netif: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<()> {
    let mut wifi_netif = wifi_netif.lock().unwrap();
    let nvs = nvs_config.lock().unwrap();

    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_SSID)?,
        password: NvsWifi::get_field::<64>(&nvs, NvsKeys::STA_PASSWD)?,
        auth_method: AuthMethod::from_str(NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_AUTH_METHOD)?.as_str())?,
        ..Default::default()
    });

    wifi_netif.set_configuration(&wifi_config)?;

    Ok(())
}

pub fn start(wifi_netif: Arc<Mutex<EspWifi<'static>>>) -> anyhow::Result<()> {
    let mut lock = wifi_netif.lock().unwrap();
    lock.start()?;

    Ok(())
}

pub fn connect(wifi_netif: Arc<Mutex<EspWifi<'static>>>) -> anyhow::Result<()> {
    let mut lock = wifi_netif.lock().unwrap();
    lock.connect()?;

    Ok(())
}

pub fn disconnect(wifi_netif: Arc<Mutex<EspWifi<'static>>>) -> anyhow::Result<()> {
    let mut lock = wifi_netif.lock().unwrap();
    lock.disconnect()?;

    Ok(())
}
