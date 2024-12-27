use std::sync::{Arc, Mutex};

use anyhow::Error;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer, Method};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::WgConfig;

mod assets_routes;
mod index;
mod wg_routes;
mod wifi_routes;

pub fn start_http_server(
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<EspHttpServer<'static>> {
    let mut http_server = EspHttpServer::new(&HttpServerConfig {
        http_port: 80,
        ..Default::default()
    })?;

    assets_routes::set_routes(&mut http_server)?;
    wifi_routes::set_routes(&mut http_server, Arc::clone(&nvs), Arc::clone(&wifi))?;
    wg_routes::set_routes(&mut http_server, Arc::clone(&nvs), Arc::clone(&wifi))?;

    // Handler to get the main config page
    http_server.fn_handler("/", Method::Get, {
        let nvs = Arc::clone(&nvs);
        move |mut request| {
            let wg_conf = WgConfig::get_config(Arc::clone(&nvs))?;

            let html = index::index_html(&wg_conf)?;

            let connection = request.connection();

            connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

            connection.write(html.as_bytes())?;

            Ok::<(), Error>(())
        }
    })?;

    Ok(http_server)
}
