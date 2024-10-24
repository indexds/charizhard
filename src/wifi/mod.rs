use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use crate::env::Env;
use log::info;

pub fn start_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let env = Env::new()?;

    let sta_configuration = Configuration::Client(ClientConfiguration {
        ssid: env.sta_ssid.try_into()?,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: env.sta_passwd.try_into()?,
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