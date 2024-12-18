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

    let _eth_netif = eth::init_eth(peripherals.pins, peripherals.mac, sysloop.clone())?;

    let wifi_netif = wifi::init_wifi(peripherals.modem, sysloop.clone(), nvs.clone())?;

    let (_http, _mdns) = http::start_http_server(nvs_config.clone(), wifi_netif.clone())?;

    std::thread::park();

    Ok(())
}
