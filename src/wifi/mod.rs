use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{AccessPointConfiguration, BlockingWifi, EspWifi};
use crate::env::Env;
use log::info;

use heapless::String as heapless_string;

pub fn start_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let env = Env::new()?;

    let sta_configuration = ClientConfiguration {
        ssid: env.sta_ssid.try_into()?,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: env.sta_passwd.try_into()?,
        channel: None,
        ..Default::default()
    };

    //temp
    let mut heapless_ssid = heapless_string::<32>::new();
    let mut heapless_passwd = heapless_string::<64>::new();

    heapless_ssid.push_str("charizhard").unwrap();
    heapless_passwd.push_str("test").unwrap();
    

    let ap_configuration = AccessPointConfiguration {
        ssid: heapless_ssid,
        password: heapless_passwd,
        channel: 1,
        max_connections: 4,
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    
    };

    let mixed_config = Configuration::Mixed(sta_configuration, ap_configuration);
    //end temp

    wifi.set_configuration(&mixed_config)?;

    wifi.start()?;
    info!("WIFI STARTED..");

    wifi.connect()?;
    info!("WIFI CONNECTED.");

    wifi.wait_netif_up()?;
    info!("WIFI NETIF UP.");

    Ok(())
}