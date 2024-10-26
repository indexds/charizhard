use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::timer::EspTimerService;
use esp_idf_svc::wifi::{AsyncWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use async_executor::Executor;
use std::sync::Arc;
use std::time::Duration;
use log::info;

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
    let timer = EspTimerService::new()?;

    let executor = Arc::new(Executor::new());

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(peripherals.modem, event_loop.clone(), Some(nvs))?,
        event_loop,
        timer,
    )?;

    executor.spawn(async move {
        match wifi::start_wifi(&mut wifi).await {
            Ok(_) => info!("wifi started"),
            Err(e) => info!("wifi error: {:?}", e),
        }
    }).detach();
    

    let (http_server, mdns) = crate::http::start_http_server()?;

    loop {
        std::thread::sleep(Duration::from_millis(100));
    }
    
}
