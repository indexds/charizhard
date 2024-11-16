use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use std::sync::{Arc, Mutex};
use utils::nvs::{NvsKeys, NvsWifi};

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

    let (_http_server, _mdns) = http::start_http_server(Arc::clone(&guarded_nvs), Arc::clone(&guarded_wifi))?;

    // WG TEST
    let mut nvslock = guarded_nvs.try_lock().unwrap();
    NvsWifi::set_field(&mut nvslock, NvsKeys::STA_PASSWD, "fishingrodent")?;
    NvsWifi::set_field(&mut nvslock, NvsKeys::STA_PASSWD, "iliketrains")?;
    NvsWifi::set_field(&mut nvslock, NvsKeys::STA_AUTH_METHOD, "wpa2personal")?;
    drop(nvslock);
    wifi::connect_wifi(&Arc::clone(&guarded_wifi), &Arc::clone(&guarded_nvs))?;
    let _ctx = wireguard::start_wg_tunnel(Arc::clone(&guarded_nvs))?;
    // END WG TEST

    loop {
        std::thread::park();
    }
}
