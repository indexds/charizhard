use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_hal::io::Write;
use anyhow::Error;


#[allow(unused_must_use)]
pub fn set_routes(mut http_server: EspHttpServer<'static>) -> EspHttpServer<'static> {
    
    http_server.fn_handler("/index.js", Method::Get, move |mut request| {

        let javascript = include_str!("./index.js");
        
        let connection = request.connection();
        
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "application/javascript")
        ])?;
        
        connection.write(javascript.as_bytes())?;
    
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/index.css", Method::Get, move |mut request| {
    
        let css = include_str!("./index.css");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "text/css")
        ])?;
        
        connection.write(css.as_bytes())?;
    
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/spinner.svg", Method::Get, move |mut request| {
    
        let spinner = include_str!("./assets/spinner.svg");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
        ])?;
    
        connection.write(spinner.as_bytes())?;
        
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/signal-1.svg", Method::Get, move |mut request| {
    
        let signal_one = include_str!("./assets/signal-1.svg");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
        ])?;
    
        connection.write(signal_one.as_bytes())?;
        
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/signal-2.svg", Method::Get, move |mut request| {
    
        let signal_two = include_str!("./assets/signal-2.svg");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
        ])?;
    
        connection.write(signal_two.as_bytes())?;
        
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/signal-3.svg", Method::Get, move |mut request| {
    
        let signal_three = include_str!("./assets/signal-3.svg");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
        ])?;
    
        connection.write(signal_three.as_bytes())?;
        
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/signal-4.svg", Method::Get, move |mut request| {
    
        let signal_four = include_str!("./assets/signal-4.svg");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
        ])?;
    
        connection.write(signal_four.as_bytes())?;
        
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/unlocked.svg", Method::Get, move |mut request| {
    
        let unlocked = include_str!("./assets/unlocked.svg");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
        ])?;
        
        connection.write(unlocked.as_bytes())?;
        
        Ok::<(), Error>(())
    });
    
    http_server.fn_handler("/locked.svg", Method::Get, move |mut request| {
    
        let locked = include_str!("./assets/locked.svg");
    
        let connection = request.connection();
    
        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
        ])?;
        
        connection.write(locked.as_bytes())?;
        
        Ok::<(), Error>(())
    });

    http_server
}