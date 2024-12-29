use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use crate::utils::nvs::WgConfig;

const FAVICON_DATA: &[u8] = include_bytes!("./static/assets/favicon.ico");

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
                            <input type="text" id="address" name="address" value="{}" placeholder="e.g. 72.84.134.96" required>
                            <div class="error" id="address-error"></div>

                            <label for="port">Port</label>
                            <input type="text" id="port" name="port" value="{}" placeholder="e.g. 51820" required>
                            <div class="error" id="port-error"></div>

                            <label for="privkey">Client Private Key</label>
                            <input type="password" id="privkey" name="privkey" value="{}" placeholder="e.g. mymtN3XjUj/UkbZkIPI1X28=" required>
                            <div class="error" id="privkey-error"></div>

                            <label for="pubkey">Server Public Key</label>
                            <input type="text" id="pubkey" name="pubkey" value="{}" placeholder="e.g. vBTj0TgQpQzjBWEShTkd8AU=" required>
                            <div class="error" id="pubkey-error"></div>

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
        wg_config.address.as_str(),
        wg_config.port.as_str(),
        wg_config.client_private_key.as_str(),
        wg_config.server_public_key.as_str(),
    ))
}
