use heapless::String as HeaplessString;

#[derive(Debug)]
pub struct EnvVars{
    pub ssid: HeaplessString<32>,
    pub password: HeaplessString<64>,
    pub _tcp_listening_port: u16,
}

impl EnvVars{
    fn env_var(env_var: &str) -> Result<&str, anyhow::Error>{
        let var = include_str!(".env").lines()
                .find(|line| line.starts_with(&env_var))
                .unwrap_or("")
                .split('=')
                .nth(1)
                .unwrap_or("");
        
        Ok(var)
    }

    pub fn new() -> Result<Self, anyhow::Error>{
        
        let ssid_env = EnvVars::env_var("WIFI_SSID=")?;
        let password_env = EnvVars::env_var("WIFI_PASSWORD=")?;
        let tcp_listening_port_env = EnvVars::env_var("TCP_LISTENING_PORT=")?;
        
        let mut ssid: HeaplessString<32> = HeaplessString::new();
        let mut password: HeaplessString<64> = HeaplessString::new();
        
        let _tcp_listening_port = tcp_listening_port_env.parse::<u16>().unwrap_or(5000);

        let _ = ssid.push_str(&ssid_env);
        let _ = password.push_str(&password_env);
        
        Ok(Self { 
            ssid,
            password, 
            _tcp_listening_port,
        })
    }   
}