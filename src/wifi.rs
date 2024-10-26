use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{AccessPointConfiguration, AsyncWifi, EspWifi};
use log::info;
use crate::utils::nvs::Nvs;

//temp
use crate::utils::heapless::HeaplessString;

pub async fn start_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let env = Nvs::new()?;

    let sta_configuration = ClientConfiguration {
        ssid: env.sta_ssid.try_into()?,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: env.sta_passwd.try_into()?,
        channel: None,
        ..Default::default()
    };


    //temp
    let mut heapless_ssid = HeaplessString::<32>::new();
    let mut heapless_passwd = HeaplessString::<64>::new();

    heapless_ssid.push_str("charizhard")?;
    heapless_passwd.push_str("testpassword")?;

    let ap_configuration = AccessPointConfiguration {
        ssid: heapless_ssid.try_into()?,
        password: heapless_passwd.try_into()?,
        channel: 1,
        max_connections: 4,
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    
    };

    let mixed_config = Configuration::Mixed(sta_configuration, ap_configuration);
    //end temp

    wifi.set_configuration(&mixed_config)?;

    wifi.start().await?;
    info!("WIFI STARTED..");

    wifi.connect().await?;
    info!("WIFI CONNECTED.");

    wifi.wait_netif_up().await?;
    info!("WIFI NETIF UP.");

    Ok(())
}