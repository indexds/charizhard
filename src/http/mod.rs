use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfig, Method};
use esp_idf_svc::mdns::EspMdns;
use anyhow::Error;

mod html;

#[allow(unused_must_use)]
pub fn start_http_server() -> anyhow::Result<(EspHttpServer<'static>, EspMdns)> {
    

    let http_config = HttpServerConfig {
        http_port: 80,
        https_port: 443,
        ..Default::default()
    };

    let mut http_server = EspHttpServer::new(&http_config)?;

    http_server.fn_handler("/", Method::Get, |request| {

        let html = html::index_html();

        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;
        Ok::<(), Error>(())
    })?;

    http_server.fn_handler("/#", Method::Get, |request| {
        
        let html = html::submit();

        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;
        Ok::<(), Error>(())
    });

    let mut mdns = EspMdns::take()?;
    mdns.set_hostname("charizhard")?;
    mdns.add_service(Some("_http"), "_tcp", "80", 60, &[])?;
    mdns.add_service(Some("_https"), "_tcp", "443", 60, &[])?;

    Ok((http_server, mdns))
}