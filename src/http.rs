use base64::prelude::BASE64_STANDARD;
use esp_idf_svc::mdns::EspMdns;

use esp_idf_svc::http::server::{EspHttpServer, Configuration as HttpServerConfig, Method};
use anyhow::Error;
use base64::Engine;

fn base64_favicon() -> String {
    let favicon_data = include_bytes!("C:/chhard/favicon.ico");
    let favicon_base64 = BASE64_STANDARD.encode(favicon_data);

    favicon_base64
}

fn index_html() -> String {

    let favicon = base64_favicon();

    format!(
            r###"
            <!DOCTYPE html>
            <html>
                <head>
                    <meta charset="utf-8">
                    <title>Charizhard</title>
                    <link rel="icon" type="image/png" href="data:image/png;base64,{favicon}">
                </head>
                <body>
                    <form action="#" method="post">
                        <label for="user-input">Enter something:</label>
                        <input type="text" id="user-input" name="user-input">
                        <button type="submit">Submit</button>
                    </form>
                </body>
            </html>
            "###
    )
}

pub fn start_http_server() -> anyhow::Result<()> {

    let mut http_server = EspHttpServer::new(&HttpServerConfig::default())?;

    http_server.fn_handler("/", Method::Get, |request| {

        let html = index_html();

        let mut response = request.into_ok_response()?;

        response.write(html.as_bytes())?;
        Ok::<(), Error>(())
    })?;

    let mut mdns = EspMdns::take()?;
    mdns.set_hostname("charizhard")?;
    mdns.add_service(Some("_http"), "_tcp", "80", 60, &[])?;

    Ok(())
}