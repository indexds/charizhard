use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use crate::utils::heapless::HeaplessString;
use crate::utils::nvs::WgConfig;

/// Stores the data for the http server's favicon as a byte slice to be included
/// in rendering.
const FAVICON_DATA: &[u8] = include_bytes!("./static/assets/favicon.ico");

fn get_value<'a, const N: usize>(remember_me: &'a HeaplessString<8>, value: &'a HeaplessString<N>) -> &'a str {
    if remember_me.0.parse::<bool>().unwrap_or(false) {
        value.as_str()
    } else {
        ""
    }
}

/// Gives the html for the "/" handler, with respect to the current wireguard
/// configuration (autofill).
pub fn index_html(wg_config: &WgConfig) -> anyhow::Result<String> {
    let favicon = BASE64_STANDARD.encode(FAVICON_DATA);

    Ok(format!(
        r###"
            <!DOCTYPE html>
            <html lang="en">
                
                <head>
                    <link rel="icon" type="image/png" href="data:image/png;base64,{favicon}">
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Charizhard</title>
                    <link rel="stylesheet" href="index.css">
                </head>
                
                <body>
                    <div class="top-container">
                        <h1>Wireguard</h1>
                        
                        <form id="config" method="post" action="/connect-wg">
                            <label for="address">Endpoint</label>
                            <input type="text" id="address" name="address" value="{address}" placeholder="e.g. 72.84.134.96" required>
                            <div class="error" id="address-error"></div>

                            <label for="port">Port</label>
                            <input type="text" id="port" name="port" value="{port}" placeholder="e.g. 51820" required>
                            <div class="error" id="port-error"></div>

                            <label for="privkey">Client Private Key</label>
                            <input type="password" id="privkey" name="privkey" value="{privkey}" placeholder="e.g. mymtN3XjUj/UkbZkIPI1X28=" required>
                            <div class="error" id="privkey-error"></div>

                            <label for="pubkey">Server Public Key</label>
                            <input type="text" id="pubkey" name="pubkey" value="{pubkey}" placeholder="e.g. vBTj0TgQpQzjBWEShTkd8AU=" required>
                            <div class="error" id="pubkey-error"></div>

                            <div class="checkbox-container">
                                <input type="checkbox" id="remember-me" name="rember" {checkstate}>
                                <label for="remember-me">Remember me</label>
                            </div>

                            <button type="submit">Connect</button>
                        </form>
                    </div>
                    
                    <div class="top-container">
                        <h1>Status</h1>
                        
                        <div class="wireguard-status-block">
                            <div class="subtitle">Wireguard</div>
                            <div class="status" id="wireguard-status"></div>
                        </div>
                        
                        <div class="wifi-status-block">
                            <div class="subtitle">Wi-Fi</div>
                            <div class="status" id="wifi-status"></div>
                        </div>
                    </div>
                    
                    <div class="top-container">
                        <h1>Wi-Fi</h1>
                        <div id="scanned-wifis" class="scrollable-box">
                            <div id="inner-scanned-wifis"></div>
                            <img id="loading-svg" src="spinner.svg" alt="Loading...">
                        </div>
                        <button onclick="fetchScannedWifis()">Scan</button>
                    </div>
                </body>
                <script src="index.js"></script>
            </html>
        "###,
        favicon = favicon,
        address = get_value(&wg_config.remember_me, &wg_config.address),
        port = get_value(&wg_config.remember_me, &wg_config.port),
        privkey = get_value(&wg_config.remember_me, &wg_config.client_private_key),
        pubkey = get_value(&wg_config.remember_me, &wg_config.server_public_key),
        checkstate = if wg_config.remember_me.0.parse::<bool>().unwrap_or(false) {
            "checked"
        } else {
            ""
        },
    ))
}
