use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfig, Method};
use esp_idf_svc::mdns::EspMdns;
use anyhow::Error;
use serde::Deserialize;
use serde_urlencoded;
use crate::utils::heapless::HeaplessString;

mod html;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Config {
    sta_ssid: HeaplessString<32>,
    sta_passwd: HeaplessString<64>,
    wg_addr: HeaplessString<32>,
    wg_port: HeaplessString<16>,
    wg_dns: HeaplessString<32>,
    wg_psk_client: HeaplessString<32>,
    wg_psk_pub_server: HeaplessString<32>,
}

#[allow(unused_must_use)]
pub fn start_http_server() -> anyhow::Result<(EspHttpServer<'static>, EspMdns)> {

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

    http_server.fn_handler("/", Method::Get, |request| {

        let html = html::index_html()?;

        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/config", Method::Post, |mut request| {
        // Parse the form data
        let mut body = Vec::new();
        let mut buffer = [0_u8; 16];

        loop {
            match request.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => body.extend_from_slice(&buffer[..n]),
                Err(e) => return Err(e.into()),
            }
        }
        
        let form_data = String::from_utf8(body)?;
        let config: Config = serde_urlencoded::from_str(form_data.as_str())?;
    
        // Log the parsed configuration (you can replace this with writing to your config struct)
        println!("Received configuration: {:?}", config);
    
        // Return a success response
        let mut response = request.into_ok_response()?;
        response.write_all(b"Configuration saved successfully")?;
    
        Ok::<(), Error>(())
    });


    Ok((http_server, mdns))
}