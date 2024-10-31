use anyhow::Error;
use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::{EspHttpServer, Method};

#[allow(unused_must_use)]
pub fn set_routes(mut http_server: EspHttpServer<'static>) -> EspHttpServer<'static> {
    http_server.fn_handler("/index.js", Method::Get, move |mut request| {
        let javascript = include_str!("./index.js");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "application/javascript")],
        )?;

        connection.write(javascript.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/index.css", Method::Get, move |mut request| {
        let css = include_str!("./index.css");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[("Content-Type", "text/css")])?;

        connection.write(css.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/spinner.svg", Method::Get, move |mut request| {
        let spinner = include_str!("./assets/spinner.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(spinner.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/signal-1.svg", Method::Get, move |mut request| {
        let signal_one = include_str!("./assets/signal-1.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(signal_one.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/signal-2.svg", Method::Get, move |mut request| {
        let signal_two = include_str!("./assets/signal-2.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(signal_two.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/signal-3.svg", Method::Get, move |mut request| {
        let signal_three = include_str!("./assets/signal-3.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(signal_three.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/signal-4.svg", Method::Get, move |mut request| {
        let signal_four = include_str!("./assets/signal-4.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(signal_four.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/unlocked.svg", Method::Get, move |mut request| {
        let unlocked = include_str!("./assets/unlocked.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(unlocked.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/locked.svg", Method::Get, move |mut request| {
        let locked = include_str!("./assets/locked.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(locked.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/connected.svg", Method::Get, move |mut request| {
        let connected = include_str!("./assets/connected.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(connected.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server.fn_handler("/disconnected.svg", Method::Get, move |mut request| {
        let disconnected = include_str!("./assets/disconnected.svg");

        let connection = request.connection();

        connection.initiate_response(200, Some("OK"), &[
            ("Content-Type", "image/svg+xml"),
            ("Cache-Control", "public, max-age=3600"),
        ])?;

        connection.write(disconnected.as_bytes())?;

        Ok::<(), Error>(())
    });

    http_server
}
