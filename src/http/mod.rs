use std::sync::{Arc, Mutex};

use anyhow::Error;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpConnection, EspHttpServer, Method, Request};
use esp_idf_svc::ipv4::Ipv4Addr;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::WgConfig;

/// Handles static routes (svgs, css, javascript).
mod assets_routes;
/// Handles the main page route.
mod index;
/// Handles wireguard related routes.
mod wg_routes;
/// Handles wifi related routes.
mod wifi_routes;

use super::net::ETH_GATEWAY;

/// Checks that the source ip of the request is [`ETH_GATEWAY`] + 1. This
/// function should be called at the beginning of every call to `fn_handler` to
/// prevent security vulnerabilities.
fn check_ip(request: &mut Request<&mut EspHttpConnection>) -> anyhow::Result<()> {
    let source_ip = request.connection().raw_connection()?.source_ipv4()?;

    // This IP will be the only one allowed to access the http server once it is
    // up. By default, this is set to the DHCP address allocated to the computer
    // connecting to the esp32.
    if source_ip != Ipv4Addr::from(u32::from(ETH_GATEWAY) + 1) {
        log::warn!("Forbidden ip [{}] tried to connect! Returned 403.", source_ip);
        return Err(Error::msg("Forbidden"));
    }

    Ok(())
}

/// Starts the http server.
pub fn start(
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<EspHttpServer<'static>> {
    let mut http_server = EspHttpServer::new(&HttpServerConfig {
        http_port: 80,
        ..Default::default()
    })?;

    assets_routes::set_routes(&mut http_server)?;
    wg_routes::set_routes(&mut http_server, Arc::clone(&nvs))?;
    wifi_routes::set_routes(&mut http_server, Arc::clone(&nvs), Arc::clone(&wifi))?;

    // Handler to get the main config page
    http_server.fn_handler("/", Method::Get, {
        let nvs = Arc::clone(&nvs);
        move |mut request| {
            self::check_ip(&mut request)?;

            let connection = request.connection();

            let wg_conf = WgConfig::get_config(Arc::clone(&nvs))?;

            let html = index::index_html(&wg_conf)?;

            connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

            connection.write(html.as_bytes())?;

            Ok::<(), Error>(())
        }
    })?;

    Ok(http_server)
}
