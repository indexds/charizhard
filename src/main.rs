use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::{EspNvs, EspDefaultNvsPartition};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

// use esp_idf_svc::sys::esp_get_free_heap_size;
// use log::info;

mod wifi;
mod http;
mod utils;


#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();
    
    let peripherals = Peripherals::take()?;
    let event_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, event_loop.clone(), Some(nvs.clone()))?,
        event_loop,
    )?;
    
    wifi::start_wifi(&mut wifi)?;

    let nvs_instance = EspNvs::new(nvs.clone(), "config", true)?;
    let nvs_config = Arc::new(Mutex::new(nvs_instance));    
    // let nvs_instance = Arc::new(Mutex::new(Nvs::new(nvs_config)?));

    let (http_server, mdns) = http::start_http_server(nvs_config)?;

    // let free_heap_size = unsafe {esp_get_free_heap_size()};
    // info!("free heap: {}", free_heap_size);

    loop {
        std::thread::sleep(Duration::from_millis(100));
    }
    
}
