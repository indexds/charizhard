use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{AccessPointConfiguration, BlockingWifi, EspWifi};
use crate::env::EnvVars;
use log::info;

pub fn start_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let env = EnvVars::new()?;

    let sta_configuration = ClientConfiguration {
        ssid: env.sta_ssid,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: env.sta_passwd,
        channel: None,
        ..Default::default()
    };

    let ap_configuration = AccessPointConfiguration{
        ssid: env.ap_ssid,
        auth_method: AuthMethod::WPA2Personal,
        password: env.ap_passwd,
        ..Default::default()
    };

    let dual_configuration = Configuration::Mixed(
        ClientConfiguration{
            ..sta_configuration
        },
        AccessPointConfiguration{
            ..ap_configuration
        }
    );

    wifi.set_configuration(&dual_configuration)?;

    wifi.start()?;
    info!("WIFI STARTED..");

    wifi.connect()?;
    info!("WIFI CONNECTED.");

    wifi.wait_netif_up()?;
    info!("WIFI NETIF UP.");

    Ok(())
}