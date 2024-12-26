use std::sync::{Arc, Mutex};

use anyhow::Error;
use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::{NvsKeys, NvsWireguard};
use crate::wireguard;

pub fn set_routes(
    http_server: &mut EspHttpServer<'static>,
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<()> {
    let nvs_start_wg = Arc::clone(&nvs);
    let wifi_start_wg = Arc::clone(&wifi);
    http_server.fn_handler("/connect-wg", Method::Post, move |mut request| {
        let wifi = wifi_start_wg.lock().unwrap();
        if !wifi.is_connected()? {
            log::error!("Wifi not connected!");
            return Err(anyhow::anyhow!("Wifi not connected!"));
        }
        drop(wifi);

        let mut nvs = nvs_start_wg.lock().unwrap();
        let mut body = Vec::new();
        let mut buffer = [0u8; 128];

        loop {
            match request.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => body.extend_from_slice(&buffer[..n]),
                Err(e) => return Err(e.into()),
            }
        }

        let form_data = String::from_utf8(body)?;
        let wg_conf: NvsWireguard = serde_urlencoded::from_str(form_data.as_str())?;

        NvsWireguard::set_field(&mut nvs, NvsKeys::WG_ADDR, wg_conf.wg_addr.clean_string().as_str())?;
        NvsWireguard::set_field(&mut nvs, NvsKeys::WG_PORT, wg_conf.wg_port.clean_string().as_str())?;
        NvsWireguard::set_field(&mut nvs, NvsKeys::WG_CLI_PRI, wg_conf.wg_cli_pri.clean_string().as_str())?;
        NvsWireguard::set_field(&mut nvs, NvsKeys::WG_SERV_PUB, wg_conf.wg_serv_pub.clean_string().as_str())?;

        drop(nvs);

        // TODO! async
        // TODO! cant connect without first connecting to wifi
        wireguard::sync_sntp(Arc::clone(&wifi_start_wg))?;
        wireguard::start_wg_tunnel(Arc::clone(&nvs_start_wg))?;

        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    })?;

    http_server.fn_handler("/disconnect-wg", Method::Post, move |mut request| {
        wireguard::end_wg_tunnel()?;

        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    })?;

    http_server.fn_handler("/wg-status", Method::Get, move |mut request| {
        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

        let mut html = String::new();
        let connected_server = "Disconnected"; // temp

        let svg_status = match connected_server {
            "Disconnected" => "disconnected",
            _ => "connected",
        };

        html.push_str(
            format!(
                r###"
            <div class=svg-status-text-container>
                <img id="{}-svg-wg" src="{}.svg">
                <div id="wg-status-text">{}</div>
            </div>                
        "###,
                svg_status, svg_status, connected_server,
            )
            .as_str(),
        );

        connection.write(html.as_bytes())?;
        Ok::<(), Error>(())
    })?;

    Ok(())
}
