use crate::utils::nvs::{NvsKeys, NvsWireguard};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use anyhow::Error;
use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use std::sync::{Arc, Mutex};

#[allow(unused_must_use)]
pub fn set_routes(
    http_server: &mut EspHttpServer<'static>,
    nvs: &Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: &Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
) -> anyhow::Result<()> {

    let nvs_save_wireguard = Arc::clone(&nvs);
    http_server.fn_handler("/connect-wg", Method::Post, move |mut request| {
        let mut nvs = nvs_save_wireguard
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;

        let mut body = Vec::new();
        let mut buffer = [0_u8; 128];

        loop {
            match request.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => body.extend_from_slice(&buffer[..n]),
                Err(e) => return Err(e.into()),
            }
        }

        let form_data = String::from_utf8(body)?;
        let wg_config: NvsWireguard = serde_urlencoded::from_str(form_data.as_str())?;

        NvsWireguard::set_field(&mut nvs, NvsKeys::WG_ADDR, wg_config.wg_addr.clean_string().as_str())?;
        NvsWireguard::set_field(&mut nvs, NvsKeys::WG_PORT, wg_config.wg_port.clean_string().as_str())?;
        NvsWireguard::set_field(
            &mut nvs,
            NvsKeys::WG_CLIENT_PRIV_KEY,
            wg_config.wg_client_priv_key.clean_string().as_str(),
        )?;
        NvsWireguard::set_field(
            &mut nvs,
            NvsKeys::WG_SERVER_PUB_KEY,
            wg_config.wg_server_pub_key.clean_string().as_str(),
        )?;

        //WIREGUARD BULLSHITTERY

        //END WIREGUARD BULLSHITTERY


        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    });

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

        connection.write(html.as_bytes());
        Ok::<(), Error>(())
    });

    Ok(())
}
