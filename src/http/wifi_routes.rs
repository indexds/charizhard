use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Error;
use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::wifi::{AuthMethod, EspWifi};

use crate::network::wifi;
use crate::utils::nvs::{NvsKeys, NvsWifi};

pub fn set_routes(
    http_server: &mut EspHttpServer<'static>,
    nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
    wifi: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<()> {
    
    http_server.fn_handler("/disconnect-wifi", Method::Get, {
        let wifi_disconnect = Arc::clone(&wifi);
        move |mut request| {

        wifi::disconnect(Arc::clone(&wifi_disconnect))?;
  
        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    }})?;

    let nvs_connect = Arc::clone(&nvs);
    let wifi_connect = Arc::clone(&wifi);
    http_server.fn_handler("/connect-wifi", Method::Post, move |mut request| {
        let mut nvs_save = nvs_connect.lock().unwrap();
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
        let wifi_conf: NvsWifi = serde_urlencoded::from_str(form_data.as_str())?;

        NvsWifi::set_field(&mut nvs_save, NvsKeys::STA_SSID, wifi_conf.sta_ssid.clean_string().as_str())?;
        NvsWifi::set_field(&mut nvs_save, NvsKeys::STA_PASSWD, wifi_conf.sta_passwd.clean_string().as_str())?;
        NvsWifi::set_field(&mut nvs_save, NvsKeys::STA_AUTH, wifi_conf.sta_auth.clean_string().as_str())?;

        drop(nvs_save);

        wifi::set_configuration(Arc::clone(&nvs_connect), Arc::clone(&wifi_connect))?;
        wifi::connect(Arc::clone(&wifi_connect))?;

        let connection = request.connection();

        connection.initiate_response(204, Some("OK"), &[("Content-Type", "text/html")])?;

        Ok::<(), Error>(())
    })?;

    let wifi_get = Arc::clone(&wifi);
    http_server.fn_handler("/wifi", Method::Get, move |request| {
        let mut wifi = wifi_get.lock().unwrap();

        if !wifi.is_started()? {
            wifi.start()?;
        }

        let mut scanned = wifi.scan()?;

        // Remove dups
        scanned.sort_by(|a, b| a.ssid.cmp(&b.ssid));
        scanned.dedup_by(|a, b| a.ssid == b.ssid);

        // Sort by desc sig strength (values are negative, with 0db max, -50db avg)
        scanned.sort_by(|a, b| b.signal_strength.cmp(&a.signal_strength));

        let mut html = String::new();

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
                        <input type='hidden' id='passwd-{}' name='passwd' value="">
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
                            <form id='connect-form-{}' method='post' action='/connect-wifi'>
                                <input type='hidden' name='authmethod' value='{}'>
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
                    access_point.auth_method.unwrap_or_default(),
                    &access_point.ssid,
                    password_html,
                )
                .as_str(),
            );
        }

        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;

        Ok::<(), Error>(())
    })?;

    let wifi_status = Arc::clone(&wifi);
    http_server.fn_handler("/wifi-status", Method::Get, move |mut request| {
        let wifi = wifi_status.lock().unwrap();

        let is_connected = wifi.is_connected()?;

        let binding = wifi.get_configuration()?;

        let ssid = match binding.as_client_conf_ref() {
            Some(config) => {
                if is_connected {
                    config.ssid.as_str()
                } else {
                    "Disconnected"
                }
            }
            None => "Disconnected",
        };

        let svg_status = match is_connected {
            true => "connected",
            false => "disconnected",
        };

        let mut html = String::new();

        html.push_str(
            format!(
                r###"
                <div class=svg-status-text-container>
                    <img id="{}-svg-wifi" src="{}.svg">
                    <div id="wifi-status-text">{}</div>
                </div>
            "###,
                svg_status, svg_status, ssid,
            )
            .as_str(),
        );

        if is_connected {
            html.push_str(
                r###"
                <button id="disconnect-wifi-button" onclick="disconnectWifi()">Disconnect</button>
                "###,
            );
        }

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/html")])?;

        connection.write(html.as_bytes())?;

        Ok::<(), Error>(())
    })?;

    Ok(())
}
