use base64::prelude::BASE64_STANDARD;
use crate::utils::nvs::Nvs;
use base64::Engine;


const FAVICON_DATA: &'static [u8] = include_bytes!("favicon.ico");


pub fn index_html(nvs: &Nvs) -> anyhow::Result<String> {
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
                    <h1>Configuration</h1>
                    
                    <form id="config" method="post" action="/save">
                        <label for="ssid">Wi-Fi SSID</label>
                        <input type="text" id="ssid" name="ssid" value="{}" required>

                        <label for="passwd">Wi-Fi Password</label>
                        <input type="text" id="passwd" name="passwd" value="{}" required>

                        <label for="address">WireGuard Address</label>
                        <input type="text" id="address" name="address" value="{}" required>

                        <label for="port">WireGuard Port</label>
                        <input type="text" id="port" name="port" value="{}">

                        <label for="dns">WireGuard DNS</label>
                        <input type="text" id="dns" name="dns" value="{}">

                        <label for="privkey">Client Private Key</label>
                        <input type="text" id="privkey" name="privkey" value="{}" required>

                        <label for="pubkey">Remote Host Public Key</label>
                        <input type="text" id="pubkey" name="pubkey" value="{}" required> 
                        <button type="submit">Submit</button>
                    </form>

                    <script src="index.js"></script>
                </body>
            </html>
        "###, 
        nvs.sta_ssid.clean_string().as_str(), 
        nvs.sta_passwd.clean_string().as_str(), 
        nvs.wg_addr.clean_string().as_str(), 
        nvs.wg_port.clean_string().as_str(), 
        nvs.wg_dns.clean_string().as_str(), 
        nvs.wg_client_priv_key.clean_string().as_str(), 
        nvs.wg_server_pub_key.clean_string().as_str()
    ))
}