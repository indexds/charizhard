use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Error;
use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::{NvsKeys, NvsWireguard};
use crate::wireguard;
use crate::wireguard::ctx::WG_CTX;

pub fn set_routes(
    http_server: &mut EspHttpServer<'static>,
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<()> {
    // Handler to connect to a wireguard peer
    http_server.fn_handler("/connect-wg", Method::Post, {
        let nvs_set = Arc::clone(&nvs);
        let wifi_check = Arc::clone(&wifi);

        // This is so fucking stupid but we can't do otherwise
        let wifi = Arc::clone(&wifi);
        let nvs = Arc::clone(&nvs);

        move |mut request| {
            let wifi_check = wifi_check.lock().unwrap();

            if !wifi_check.is_connected()? {
                log::error!("Wifi not connected!");
                return Err(anyhow::anyhow!("Wifi not connected!"));
            }

            drop(wifi_check);

            let mut body = Vec::new();
            let mut buffer = [0u8; 128];

            loop {
                match request.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => body.extend_from_slice(&buffer[..n]),
                    Err(e) => return Err(e.into()),
                }
            }

            let wg_conf: NvsWireguard = serde_urlencoded::from_str(String::from_utf8(body)?.as_str())?;

            let mut nvs_set = nvs_set.lock().unwrap();

            NvsWireguard::set_field(&mut nvs_set, NvsKeys::WG_ADDR, wg_conf.wg_addr.clean_string().as_str())?;
            NvsWireguard::set_field(&mut nvs_set, NvsKeys::WG_PORT, wg_conf.wg_port.clean_string().as_str())?;
            NvsWireguard::set_field(&mut nvs_set, NvsKeys::WG_CLI_PRI, wg_conf.wg_cli_pri.clean_string().as_str())?;
            NvsWireguard::set_field(&mut nvs_set, NvsKeys::WG_SERV_PUB, wg_conf.wg_serv_pub.clean_string().as_str())?;

            drop(nvs_set);

            // Yeah..
            let wifi = Arc::clone(&wifi);
            let nvs = Arc::clone(&nvs);

            thread::spawn(move || {
                _ = wireguard::sync_sntp(Arc::clone(&wifi));
                _ = wireguard::start_wg_tunnel(Arc::clone(&nvs));
            });

            let connection = request.connection();

            connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

            Ok::<(), Error>(())
        }
    })?;

    // Handler to disconnect from the wireguard peer
    http_server.fn_handler("/disconnect-wg", Method::Post, move |mut request| {
        thread::spawn(|| {
            _ = wireguard::end_wg_tunnel();
        });

        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    })?;

    // Handler to get current wireguard status (connected/disconnected)
    // TODO! FIX THIS CALLBACK
    http_server.fn_handler("/wg-status", Method::Get, {
        let nvs = Arc::clone(&nvs);

        move |mut request| {
            let connection = request.connection();

            connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

            let mut html = String::new();

            let ctx = WG_CTX.lock().unwrap();

            // If ctx is Some, then we returned from wireguard::start_wg_tunnel so we have
            // to be connected
            let svg_status = match *ctx {
                Some(_) => "connected",
                None => "disconnected",
            };

            let nvs = nvs.lock().unwrap();
            let nvs = NvsWireguard::new(&nvs)?;

            let status = match *ctx {
                Some(_) => nvs.wg_addr.as_str(),
                None => "Disconnected",
            };

            html.push_str(
                format!(
                    r###"
                    <div class=svg-status-text-container>
                        <img id="{}-svg-wg" src="{}.svg">
                        <div id="wg-status-text">{}</div>
                    </div>                
                "###,
                    svg_status, svg_status, status,
                )
                .as_str(),
            );

            match *ctx {
                Some(_) => {
                    html.push_str(
                        r###"
                    <button id="disconnect-wg-button" onclick="disconnectWg()">Disconnect</button>
                    "###,
                    );
                }
                None => {}
            };

            connection.write(html.as_bytes())?;
            Ok::<(), Error>(())
        }
    })?;

    Ok(())
}
