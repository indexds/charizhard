use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::sync::{Arc, Mutex};

#[cfg(feature = "wireguard")]
use wireguard::context::WireguardContext;

mod bridge;
mod http;
mod utils;
mod wifi;

#[cfg(feature = "wireguard")]
mod wireguard;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let event_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, event_loop.clone(), Some(nvs.clone()))?,
        event_loop,
    )?;

    let guarded_wifi = Arc::new(Mutex::new(wifi));
    let nvs_instance = EspNvs::new(nvs.clone(), "config", true)?;
    let guarded_nvs = Arc::new(Mutex::new(nvs_instance));

    wifi::start_ap(Arc::clone(&guarded_wifi))?;

    let (_http_server, _mdns) = http::start_http_server(Arc::clone(&guarded_nvs), Arc::clone(&guarded_wifi))?;

    loop {
        std::thread::park();
    }
}
