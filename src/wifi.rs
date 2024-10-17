use crate::env_vars::EnvVars;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;

pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_ids = EnvVars::new()?;

    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: wifi_ids.ssid,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: wifi_ids.password,
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("WIFI STARTED..");

    wifi.connect()?;
    info!("WIFI CONNECTED.");

    wifi.wait_netif_up()?;
    info!("WIFI NETIF UP.");

    Ok(())
}