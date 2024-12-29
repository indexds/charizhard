use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Error;
use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::EspWifi;

use crate::utils::nvs::WgConfig;
use crate::wireguard;
use crate::wireguard::ctx::WG_CTX;

/// Sets the Wireguard related routes for the http server.
pub fn set_routes(
    http_server: &mut EspHttpServer<'static>,
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<()> {
    // Handler to connect to a wireguard peer
    http_server.fn_handler("/connect-wg", Method::Post, {
        let wifi_check = Arc::clone(&wifi);

        // This is so fucking stupid but we can't do otherwise
        let wifi = Arc::clone(&wifi);
        let nvs = Arc::clone(&nvs);

        move |mut request| {
            super::check_ip(&mut request)?;

            // We scope this to drop the lock on wifi at the end, as it needs to be locked
            // in the sync_sntp function below
            {
                let wifi_check = wifi_check.lock().unwrap();

                if !wifi_check.is_connected()? {
                    log::error!("Wifi not connected!");
                    return Err(anyhow::anyhow!("Wifi not connected!"));
                }
            }

            let mut body = Vec::new();
            let mut buffer = [0u8; 128];

            loop {
                match request.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => body.extend_from_slice(&buffer[..n]),
                    Err(e) => return Err(e.into()),
                }
            }

            let wg_conf: WgConfig = serde_urlencoded::from_str(String::from_utf8(body)?.as_str())?;

            WgConfig::set_config(Arc::clone(&nvs), wg_conf)?;

            // Yeah..
            let wifi = Arc::clone(&wifi);
            let nvs = Arc::clone(&nvs);

            thread::spawn(move || {
                _ = wireguard::sync_systime(Arc::clone(&wifi));
                _ = wireguard::start_tunnel(Arc::clone(&nvs));
            });

            let connection = request.connection();

            connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

            Ok::<(), Error>(())
        }
    })?;

    // Handler to disconnect from the wireguard peer
    http_server.fn_handler("/disconnect-wg", Method::Get, move |mut request| {
        super::check_ip(&mut request)?;

        thread::spawn(|| {
            _ = wireguard::end_tunnel();
        });

        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    })?;

    // Handler to get current wireguard status (connected/disconnected)
    http_server.fn_handler("/wg-status", Method::Get, {
        let nvs = Arc::clone(&nvs);

        move |mut request| {
            super::check_ip(&mut request)?;

            let guard = WG_CTX.lock().unwrap();
            // If ctx is (not) a null pointer, then we have to be connected to a peer
            let is_connected = guard.is_set();

            let nvs = WgConfig::get_config(Arc::clone(&nvs))?;

            let svg_status = if is_connected { "connected" } else { "disconnected" };

            let status = if is_connected {
                nvs.address.as_str()
            } else {
                "Disconnected"
            };

            let mut html = format!(
                r###"
                    <div class=svg-status-text-container>
                        <img id="{svg_status}-svg-wg" src="{svg_status}.svg">
                        <div id="wg-status-text">{status}</div>
                    </div>
                "###
            );

            if is_connected {
                html.push_str(
                    r###"
                        <button id="disconnect-wg-button" onclick="disconnectWg()">Disconnect</button>
                    "###,
                );
            }

            let connection = request.connection();

            connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

            connection.write(html.as_bytes())?;

            Ok::<(), Error>(())
        }
    })?;

    Ok(())
}
