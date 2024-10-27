use base64::prelude::BASE64_STANDARD;
use crate::utils::nvs::Nvs;
use std::sync::Arc;
use base64::Engine;


const FAVICON_DATA: &'static [u8] = include_bytes!("favicon.ico");


pub fn index_html(nvs: Arc<Nvs>) -> anyhow::Result<String> {
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
                <style>
                    
                    body {{
                        font-family: Arial, sans-serif;
                        margin: 20px;
                        padding: 20px;
                        border: 1px solid #ccc;
                        border-radius: 5px;
                        max-width: 400px;
                    }}
                    
                    h1 {{
                        text-align: center;
                    }}
                    
                    label {{
                        display: block;
                        margin: 10px 0 5px;
                    }}

                    input[type="text"] {{
                        width: 100%;
                        padding: 8px;
                        margin-bottom: 15px;
                        border: 1px solid #ccc;
                        border-radius: 4px;
                    }}
                    button {{
                        width: 100%;
                        padding: 10px;
                        background-color: #28a745;
                        border: none;
                        color: white;
                        font-size: 16px;
                        border-radius: 4px;
                        cursor: pointer;
                    }}
                    button:hover {{
                        background-color: #218838;
                    }}
                
                </style>
            </head>
            <body>

            <h1>Configuration</h1>
            
            <form id="config" method="post" action="/config">
                <label for="sta_ssid">Wi-Fi SSID:</label>
                <input type="text" id="sta_ssid" name="sta_ssid" value="{}" required>

                <label for="sta_passwd">Wi-Fi Password:</label>
                <input type="text" id="sta_passwd" name="sta_passwd" value="{}" required>

                <label for="wg_addr">WireGuard Address:</label>
                <input type="text" id="wg_addr" name="wg_addr" value="{}" required>

                <label for="wg_port">WireGuard Port:</label>
                <input type="text" id="wg_port" name="wg_port" value="{}">

                <label for="wg_dns">WireGuard DNS:</label>
                <input type="text" id="wg_dns" name="wg_dns" value="{}">

                <label for="wg_client_priv_key">Client Private Key:</label>
                <input type="text" id="wg_client_priv_key" name="wg_client_priv_key" value="{}" required>

                <label for="wg_server_pub_key">Remote Host Public Key:</label>
                <input type="text" id="wg_server_pub_key" name="wg_server_pub_key" value="{}" required>
                <button type="submit">Submit</button>

            </form>

            </body>
            </html>
        "###, 
        nvs.sta_ssid.as_str(), 
        nvs.sta_passwd.as_str(), 
        nvs.wg_addr.as_str(), 
        nvs.wg_port.as_str(), 
        nvs.wg_dns.as_str(), 
        nvs.wg_client_priv_key.as_str(), 
        nvs.wg_server_pub_key.as_str()
    ))
}