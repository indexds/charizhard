use embedded_svc::wifi::{AuthMethod, Configuration};
use esp_idf_svc::wifi::{AccessPointConfiguration, BlockingWifi, EspWifi};
use log::info;

// use embedded_svc::wifi::ClientConfiguration;
//use crate::utils::nvs::Nvs;

//temp
use crate::utils::heapless::HeaplessString;

pub fn start_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {


    // let mut sta_ssid = HeaplessString::<32>::new();
    // let mut sta_passwd = HeaplessString::<64>::new();

    // sta_ssid.push_str("fishingrodent")?;
    // sta_passwd.push_str("ijustlovepearssofuckingmuchbros")?;

    // let sta_configuration = ClientConfiguration {
    //     ssid: sta_ssid.try_into()?,
    //     bssid: None,
    //     auth_method: AuthMethod::WPA2Personal,
    //     password: sta_passwd.try_into()?,
    //     channel: None,
    //     ..Default::default()
    // };


    //temp
    let mut ap_ssid = HeaplessString::<32>::new();
    let mut ap_passwd = HeaplessString::<64>::new();

    ap_ssid.push_str("charizhard")?;
    ap_passwd.push_str("testpassword")?;

    let ap_configuration = AccessPointConfiguration {
        ssid: ap_ssid.try_into()?,
        password: ap_passwd.try_into()?,
        channel: 1,
        max_connections: 4,
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    
    };

    // let mixed_config = Configuration::Mixed(sta_configuration, ap_configuration);
    let ap_config = Configuration::AccessPoint(ap_configuration);
    //end temp

    wifi.set_configuration(&ap_config)?;

    wifi.start()?;
    info!("WIFI STARTED..");

    // wifi.connect()?;
    // info!("WIFI CONNECTED.");

    // wifi.wait_netif_up()?;
    // info!("WIFI NETIF UP.");

    Ok(())
}