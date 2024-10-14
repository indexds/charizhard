use std::time::Duration;
use crate::env_vars::EnvVars;
use esp_idf_svc::ping;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;
use std::net::Ipv4Addr;

pub fn ping(ip: Ipv4Addr) -> anyhow::Result<()> {

    let config = ping::Configuration {
        count: 5,
        interval: Duration::from_millis(1000),
        timeout: Duration::from_millis(5000),
        ..Default::default()
    };

    let summary = ping::EspPing::new(0).ping(ip, &config)?;

    info!("{:?}", summary);

    Ok(())
}


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
    info!("WIFI CONNECTED..");

    wifi.wait_netif_up()?;
    info!("WIFI NETIF UP..");
    
    let ip = "1.1.1.1".parse::<Ipv4Addr>()?;
    loop{
        ping(ip)?;
        if ip.is_loopback(){
            break;
        }
    }


    Ok(())
}