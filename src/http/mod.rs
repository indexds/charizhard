use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfig, Method};
use esp_idf_svc::mdns::EspMdns;
use anyhow::Error;
use serde_urlencoded;
use crate::utils::nvs::Nvs;
use std::sync::{Arc, Mutex};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};

mod index;

#[allow(unused_must_use)]
pub fn start_http_server(nvs: Arc<Mutex<EspNvs<NvsDefault>>>) -> anyhow::Result<(EspHttpServer<'static>, EspMdns)> {

    let nvs = nvs.lock().map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;

    let nvs_config = Arc::new(Nvs::new(nvs)?);
    
    let http_config = HttpServerConfig {
        http_port: 80,
        https_port: 443,
        ..Default::default()
    };

    let mut http_server = EspHttpServer::new(&http_config)?;

    let mut mdns = EspMdns::take()?;
    mdns.set_hostname("charizhard")?;
    mdns.add_service(Some("_http"), "_tcp", "80", 60, &[])?;
    mdns.add_service(Some("_https"), "_tcp", "443", 60, &[])?;

    http_server.fn_handler("/", Method::Get, move |request| {

        let html = index::index_html(nvs_config.clone())?;

        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/config", Method::Post, |mut request| {
        
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
        let _config: Nvs = serde_urlencoded::from_str(form_data.as_str())?;
    
        let mut response = request.into_ok_response()?;
        response.write_all(b"Configuration saved successfully")?;
    
        Ok::<(), Error>(())
    });


    Ok((http_server, mdns))
}