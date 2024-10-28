use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfig, Method};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use esp_idf_svc::mdns::EspMdns;
use std::sync::{Arc, Mutex};
use esp_idf_hal::io::Write;
use crate::utils::nvs::Nvs;
use crate::utils::nvs::NvsKeys;
use serde_urlencoded;
use anyhow::Error;


mod index;


#[allow(unused_must_use)]
pub fn start_http_server(nvs: Arc<Mutex<EspNvs<NvsDefault>>>) -> anyhow::Result<(EspHttpServer<'static>, EspMdns)> {

    let http_config = HttpServerConfig {
        http_port: 80,
        https_port: 443,
        ..Default::default()
    };

    let mut http_server = EspHttpServer::new(&http_config)?;
    
    let nvs_get = Arc::clone(&nvs);

    http_server.fn_handler("/", Method::Get, move |request| {
        
        let nvs = nvs_get.lock().map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;
        
        let nvs_config = Nvs::new(nvs)?;

        let html = index::index_html(&nvs_config)?;

        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;

        Ok::<(), Error>(())
    });

    let nvs_post = Arc::clone(&nvs);

    http_server.fn_handler("/save", Method::Post, move |mut request| {

        let mut nvs = nvs_post.lock().map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;
        
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
        let config: Nvs = serde_urlencoded::from_str(form_data.as_str())?;
        

        Nvs::set_field(&mut nvs, NvsKeys::STA_SSID, config.sta_ssid.clean_string().as_str());
        Nvs::set_field(&mut nvs, NvsKeys::STA_PASSWD, config.sta_passwd.clean_string().as_str());
        Nvs::set_field(&mut nvs, NvsKeys::WG_ADDR, config.wg_addr.clean_string().as_str());
        Nvs::set_field(&mut nvs, NvsKeys::WG_PORT, config.wg_port.clean_string().as_str());
        Nvs::set_field(&mut nvs, NvsKeys::WG_DNS, config.wg_dns.clean_string().as_str());
        Nvs::set_field(&mut nvs, NvsKeys::WG_CLIENT_PRIV_KEY, config.wg_client_priv_key.clean_string().as_str());
        Nvs::set_field(&mut nvs, NvsKeys::WG_SERVER_PUB_KEY, config.wg_server_pub_key.clean_string().as_str());

        let mut response = request.into_ok_response()?;
        response.write_all(b"Configuration saved successfully")?;
    
        Ok::<(), Error>(())
    });

    let mut mdns = EspMdns::take()?;
    mdns.set_hostname("charizhard")?;
    mdns.add_service(Some("_http"), "_tcp", "80", 60, &[])?;
    mdns.add_service(Some("_https"), "_tcp", "443", 60, &[])?;

    Ok((http_server, mdns))
}