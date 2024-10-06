use heapless::String;

#[derive(Debug)]
pub struct WifiIds{
    pub ssid: String<32>,
    pub password: String<64>,
}

impl WifiIds{
    pub fn new() -> Result<Self, anyhow::Error>{
        let mut env_contents = include_str!(".env").lines();
        
        let ssid_env = env_contents
                .find(|line| line.starts_with("WIFI_SSID="))
                .unwrap_or("")
                .split('=')
                .nth(1)
                .unwrap_or("")
                .to_string();

        let password_env = env_contents
                .find(|line| line.starts_with("WIFI_PASSWORD="))
                .unwrap_or("")
                .split('=')
                .nth(1)
                .unwrap_or("")
                .to_string();
        
        let mut ssid: String<32> = String::new();
        let mut password: String<64> = String::new();

        ssid.push_str(&ssid_env).map_err(|_| anyhow::anyhow!("SSID TOO LONG!"))?;
        password.push_str(&password_env).map_err(|_| anyhow::anyhow!("PASSWORD TOO LONG!"))?;

        Ok(Self { 
            ssid,
            password, 
        })
    }   
}