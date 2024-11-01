use crate::utils::nvs::{NvsKeys, NvsWifi, NvsWireguard};
use anyhow::Error;
use embedded_svc::wifi::AuthMethod;
use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer, Method};
use esp_idf_svc::mdns::EspMdns;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use serde_urlencoded;
use std::sync::{Arc, Mutex};

mod assets_routes;
mod index;

#[allow(unused_must_use)]
pub fn start_http_server(
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<BlockingWifi<EspWifi<'static>>>>,
) -> anyhow::Result<(EspHttpServer<'static>, EspMdns)> {
    let http_config = HttpServerConfig {
        http_port: 80,
        ..Default::default()
    };

    let mut http_server = EspHttpServer::new(&http_config)?;

    http_server = assets_routes::set_routes(http_server);

    let nvs_root = Arc::clone(&nvs);
    http_server.fn_handler("/", Method::Get, move |mut request| {
        let nvs = nvs_root
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;
        let nvs_config = NvsWireguard::new(&nvs)?;

        let html = index::index_html(&nvs_config)?;

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

        connection.write(html.as_bytes())?;

        Ok::<(), Error>(())
    });

    let nvs_save_wireguard = Arc::clone(&nvs);
    http_server.fn_handler("/save-wg", Method::Post, move |mut request| {
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

        NvsWireguard::set_field(
            &mut nvs,
            NvsKeys::WG_ADDR,
            wg_config.wg_addr.clean_string().as_str(),
        )?;
        NvsWireguard::set_field(
            &mut nvs,
            NvsKeys::WG_PORT,
            wg_config.wg_port.clean_string().as_str(),
        )?;
        NvsWireguard::set_field(
            &mut nvs,
            NvsKeys::WG_DNS,
            wg_config.wg_dns.clean_string().as_str(),
        )?;
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

        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    });

    let nvs_save_wifi = Arc::clone(&nvs);
    let nvs_connect_wifi = Arc::clone(&nvs);
    let wifi_connect = Arc::clone(&wifi);
    http_server.fn_handler("/save-wifi", Method::Post, move |mut request| {
        let mut nvs_save = nvs_save_wifi
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
        let wifi_config: NvsWifi = serde_urlencoded::from_str(form_data.as_str())?;

        NvsWifi::set_field(
            &mut nvs_save,
            NvsKeys::STA_SSID,
            wifi_config.sta_ssid.clean_string().as_str(),
        )?;
        NvsWifi::set_field(
            &mut nvs_save,
            NvsKeys::STA_PASSWD,
            wifi_config.sta_passwd.clean_string().as_str(),
        )?;

        drop(nvs_save);

        crate::wifi::connect_wifi(&wifi_connect, &nvs_connect_wifi)?;

        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    });

    let nvs_wifi_connected = Arc::clone(&wifi);
    http_server.fn_handler("/wifi-status", Method::Get, move |mut request| {
        let wifi = nvs_wifi_connected
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;

        let binding = wifi.get_configuration()?;

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

        let mut html = String::new();

        let ssid = match binding.as_client_conf_ref() {
            Some(config) => {
                if wifi.is_connected()? {
                    &config.ssid.as_str()
                } else {
                    "Disconnected"
                }
            }
            None => "Disconnected",
        };

        let svg_status = match wifi.is_connected()? {
            true => "connected",
            false => "disconnected",
        };

        html.push_str(
            format!(
                r###"
                <img id="{}-svg-wifi" src="{}.svg">
                <div id="wifi-status-text">{}</div>
            "###,
                svg_status, svg_status, ssid,
            )
            .as_str(),
        );

        connection.write(html.as_bytes());

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/wg-status", Method::Get, move |mut request| {
        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

        let mut html = String::new();
        let connected_server = "Disconnected"; //temp

        let svg_status = match connected_server {
            "Disconnected" => "disconnected",
            _ => "connected",
        };

        html.push_str(
            format!(
                r###"
                <img id="{}-svg-wg" src="{}.svg">
                <div id="wg-status-text">{}</div>
            "###,
                svg_status, svg_status, connected_server,
            )
            .as_str(),
        );

        connection.write(html.as_bytes());
        Ok::<(), Error>(())
    });

    let wifi_get = Arc::clone(&wifi);
    http_server.fn_handler("/wifi", Method::Get, move |request| {
        let mut wifi = wifi_get
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock Wifi Mutex."))?;

        let mut html = String::new();

        let mut scanned = wifi.scan()?;

        //Remove dups
        scanned.sort_by(|a, b| a.ssid.cmp(&b.ssid));
        scanned.dedup_by(|a, b| a.ssid == b.ssid);

        //Sort by desc sig strength (values are negative, with 0db max, -50db avg)
        scanned.sort_by(|a, b| b.signal_strength.cmp(&a.signal_strength));

        for access_point in scanned.iter() {
            let auth_method = match access_point.auth_method {
                Some(AuthMethod::None) => "/unlocked.svg",
                _ => "/locked.svg",
            };

            let signal_strength = match access_point.signal_strength {
                -50..=0 => "/signal-4.svg",
                -60..=-51 => "/signal-3.svg",
                -70..=-61 => "/signal-2.svg",
                _ => "/signal-1.svg",
            };

            let password_html = if access_point.auth_method != Some(AuthMethod::None) {
                format!(
                    r###"
                        <label for='passwd-{}'>Password</label>
                        <input type='password' id='passwd-{}' name='passwd' required>
                        <div class='error' id='passwd-{}-error'></div>
                    "###,
                    &access_point.ssid, &access_point.ssid, &access_point.ssid,
                )
            } else {
                format!(
                    r###"
                        <input type='hidden' id='passwd-{}' name='passwd' value=''>
                    "###,
                    &access_point.ssid,
                )
            };

            html.push_str(
                format!(
                    r###"
                    <div class='wifi' id={} onclick='toggleDropdown(event, this)'>
                        <div class='ssid-block'>    
                            <div class='ssid'>{}</div>
                            <div class='signal-auth-container'>
                                <div class='auth-method'>
                                    <img src='{}'>
                                </div>
                                <div class='signal-strength'>
                                    <img src='{}'>
                                </div>
                            </div>
                        </div>
                        <div class='wifi-connect'>
                            <form id='connect-form-{}' method='post' action='/save-wifi'>
                                <input type='hidden' name='ssid' value='{}'>
                                {}
                                <button type="submit">Connect</button>
                            </form>
                        </div>
                    </div>
                "###,
                    &access_point.ssid,
                    &access_point.ssid,
                    auth_method,
                    signal_strength,
                    &access_point.ssid,
                    &access_point.ssid,
                    password_html,
                )
                .as_str(),
            );
        }

        let mut response = request.into_ok_response()?;

        response.write_all(html.as_bytes())?;

        Ok::<(), Error>(())
    });

    let mut mdns = EspMdns::take()?;
    mdns.set_hostname("charizhard")?;
    mdns.add_service(Some("_http"), "_tcp", "80", 60, &[])?;
    mdns.add_service(Some("_https"), "_tcp", "443", 60, &[])?;

    Ok((http_server, mdns))
}
