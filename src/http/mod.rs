use std::sync::{Arc, Mutex};

use anyhow::Error;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer, Method};
use esp_idf_svc::mdns::EspMdns;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::NvsWireguard;

mod assets_routes;
mod index;
mod wg_routes;
mod wifi_routes;

pub fn start_http_server(
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<(EspHttpServer<'static>, EspMdns)> {
    let http_config = HttpServerConfig {
        http_port: 80,
        ..Default::default()
    };

    let mut http_server = EspHttpServer::new(&http_config)?;

    assets_routes::set_routes(&mut http_server)?;
    wifi_routes::set_routes(&mut http_server, Arc::clone(&nvs), Arc::clone(&wifi))?;
    wg_routes::set_routes(&mut http_server, Arc::clone(&nvs), Arc::clone(&wifi))?;

    let nvs_root = Arc::clone(&nvs);
    http_server.fn_handler("/", Method::Get, move |mut request| {
        let nvs = nvs_root.lock().unwrap();

        let nvs_wg_conf = NvsWireguard::new(&nvs)?;

        let html = index::index_html(&nvs_wg_conf)?;

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

        connection.write(html.as_bytes())?;

        Ok::<(), Error>(())
    })?;

    let mut mdns = EspMdns::take()?;
    mdns.set_hostname("charizhard")?;

    Ok((http_server, mdns))
}
