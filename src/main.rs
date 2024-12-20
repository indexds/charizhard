use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::EspNvs;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use std::sync::{Arc, Mutex};

use network::{eth, wifi};

mod http;
mod network;
mod utils;
mod wireguard;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let nvs_config = Arc::new(Mutex::new(EspNvs::new(nvs.clone(), "config", true)?));

    let wifi_driver = wifi::init_netif(peripherals.modem, sysloop.clone(), nvs.clone())?;

    wifi::set_configuration(Arc::clone(&nvs_config), Arc::clone(&wifi_driver))?;
    wifi::start(Arc::clone(&wifi_driver))?;
    wifi::connect(Arc::clone(&wifi_driver))?;

    let _eth_driver = eth::init_driver(peripherals.pins, peripherals.mac, sysloop.clone())?;

    // wireguard::start_wg_tunnel(Arc::clone(&nvs_config))?;

    let (_http, _mdns) = http::start_http_server(Arc::clone(&nvs_config), Arc::clone(&wifi_driver))?;

    std::thread::park();

    Ok(())
}
