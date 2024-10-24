use base64::Engine;
use base64::prelude::BASE64_STANDARD;

const FAVICON_DATA: &'static [u8] = include_bytes!("favicon.ico");


pub fn index_html() -> String {
    let favicon = BASE64_STANDARD.encode(FAVICON_DATA);

    format!(
        r###"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <link rel="icon" type="image/png" href="data:image/png;base64,{favicon}">
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Wi-Fi and WireGuard Configuration</title>
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

            <h1>Configuration Form</h1>
            
            <form id="configForm">
                <label for="ssid">Wi-Fi SSID:</label>
                <input type="text" id="ssid" name="ssid" value="wifi" required>

                <label for="password">Wi-Fi Password:</label>
                <input type="text" id="password" name="password" value="password" required>

                <label for="wgAddr">WireGuard Address:</label>
                <input type="text" id="wgAddr" name="wgAddr" value="0.0.0.0/24" required>

                <label for="wgPort">WireGuard Port:</label>
                <input type="text" id="wgPort" name="wgPort" value="51820" required>

                <label for="wgDns">WireGuard DNS:</label>
                <input type="text" id="wgDns" name="wgDns" value="1.1.1.1" required>

                <label for="wgPskClient">WireGuard PSK Client:</label>
                <input type="text" id="wgPskClient" name="wgPskClient" value="00000000000000000000000000000000" required>

                <label for="wgPskPubServer">WireGuard PSK Public Server:</label>
                <input type="text" id="wgPskPubServer" name="wgPskPubServer" value="00000000000000000000000000000000" required>

                <button type="submit">Submit</button>
            </form>

            </body>
            </html>
        "###
    )
}

pub fn submit() -> String {

    let favicon = BASE64_STANDARD.encode(FAVICON_DATA);

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
                Issou
            </body>
        </html>
        "###
    )
}