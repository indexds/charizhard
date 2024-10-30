use base64::prelude::BASE64_STANDARD;
use crate::utils::nvs::NvsWireguard;
use base64::Engine;

const FAVICON_DATA: &'static [u8] = include_bytes!("./assets/favicon.ico");

pub fn index_html(nvs: &NvsWireguard) -> anyhow::Result<String> {
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
                    <div class="form-container">
                        <h1>Wireguard</h1>
                        
                        <form id="config" method="post" action="/save-wg">
                            <label for="address">Address</label>
                            <input type="text" id="address" name="address" value="{}" placeholder="e.g. 0.0.0.0/24" required>
                            <div class="error" id="address-error"></div>

                            <label for="port">Port</label>
                            <input type="text" id="port" name="port" value="{}" placeholder="e.g. 51820" required>
                            <div class="error" id="port-error"></div>

                            <label for="dns">DNS</label>
                            <input type="text" id="dns" name="dns" value="{}" placeholder="e.g. 1.1.1.1" required>
                            <div class="error" id="dns-error"></div>

                            <label for="privkey">Client Private Key</label>
                            <input type="password" id="privkey" name="privkey" value="{}" placeholder="e.g. mymtN3XjUj/UkbZkIPI1X28=" required>
                            <div class="error" id="privkey-error"></div>

                            <label for="pubkey">Remote Host Public Key</label>
                            <input type="text" id="pubkey" name="pubkey" value="{}" placeholder="e.g. vBTj0TgQpQzjBWEShTkd8AU=" required>
                            <div class="error" id="pubkey-error"></div>

                            <button type="submit">Submit</button>
                        </form>
                    </div>
                    
                    <div class="wifi-container">
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
        nvs.wg_addr.clean_string().as_str(), 
        nvs.wg_port.clean_string().as_str(), 
        nvs.wg_dns.clean_string().as_str(), 
        nvs.wg_client_priv_key.clean_string().as_str(), 
        nvs.wg_server_pub_key.clean_string().as_str()
    ))
}