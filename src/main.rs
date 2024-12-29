use std::sync::{Arc, Mutex};

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::mdns::EspMdns;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
use network::{eth, wifi};

mod http;
mod network;
mod ota;
mod utils;
mod wireguard;

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let nvs_config = Arc::new(Mutex::new(EspNvs::new(nvs.clone(), "config", true)?));

    let eth_netif = eth::start(peripherals.pins, peripherals.mac, sysloop.clone())?;
    let wifi_netif = wifi::init(peripherals.modem, sysloop.clone(), nvs.clone())?;

    let http_server = http::start(Arc::clone(&nvs_config), Arc::clone(&wifi_netif))?;

    let mut mdns = EspMdns::take()?;
    mdns.set_hostname("charizhard")?;

    std::thread::park();

    Ok(())
}
