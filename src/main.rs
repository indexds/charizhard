use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use esp_idf_svc::sys::esp_get_free_heap_size;
use log::info;

mod http;
mod utils;
mod wifi;

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let event_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let nvs_instance = EspNvs::new(nvs.clone(), "config", true)?;
    let guarded_nvs = Arc::new(Mutex::new(nvs_instance));

    let wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, event_loop.clone(), Some(nvs.clone()))?,
        event_loop,
    )?;

    let mut guarded_wifi = Arc::new(Mutex::new(wifi));

    loop {
        match wifi::start_wifi(&mut guarded_wifi) {
            Ok(_) => break,
            Err(e) => {
                info!("Failed to connect to wifi. Retrying..");
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }

    let (http_server, mdns) = http::start_http_server(guarded_nvs, guarded_wifi)?;

    let free_heap_size = unsafe { esp_get_free_heap_size() };
    info!("free heap: {}", free_heap_size);

    loop {
        std::thread::sleep(Duration::from_millis(100));
    }
}
