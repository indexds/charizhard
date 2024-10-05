
use std::{
    thread::sleep,
    time::Duration,
};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{
    wifi::EspWifi,
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};

use embedded_svc::wifi::{ClientConfiguration, Wifi, Configuration};

use heapless::String;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    ).unwrap();

    let mut ssid: String<32> = String::new();
    let mut password: String<64> = String::new();
    ssid.push_str("fishingrodent").unwrap();
    password.push_str("iliketrains").unwrap();

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid,
        password,
        ..Default::default()
    })).unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    while !wifi_driver.is_connected().unwrap(){
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }
    println!("Should be connected now");
    loop{
        println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
        sleep(Duration::new(10,0));
    }

}
