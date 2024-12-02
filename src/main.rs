#![feature(never_type)]

use bridge::state::*;
use std::sync::Arc;
mod bridge;
mod http;
mod utils;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let idle = Bridge::new()?;
    let eth_ready = Bridge::<EthReady>::try_from(idle)?;
    let wifi_ready = Bridge::<WifiReady>::try_from(eth_ready)?;

    // let (_http_server, _mdns) =
    //     http::start_http_server(Arc::clone(&wifi_ready.state.nvs), Arc::clone(&wifi_ready.state.wifi))?;

    let _running = Bridge::<Running>::try_from(wifi_ready)?;

    std::thread::park();
    Ok(())
}
