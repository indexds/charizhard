use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use crate::env::EnvVars;
use log::info;

pub fn start_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let env = EnvVars::new()?;

    let sta_configuration = Configuration::Client(ClientConfiguration {
        ssid: env.sta_ssid,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: env.sta_passwd,
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&sta_configuration)?;

    wifi.start()?;
    info!("WIFI STARTED..");

    wifi.connect()?;
    info!("WIFI CONNECTED.");

    wifi.wait_netif_up()?;
    info!("WIFI NETIF UP.");

    Ok(())
}