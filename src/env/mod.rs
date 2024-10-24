mod heapless;
use heapless::HeaplessString;

const DEFAULT_STA_SSID: &str = "wifi";
const DEFAULT_STA_PASSWD: &str = "password";
const DEFAULT_WG_ADDR: &str = "0.0.0.0/24";
const DEFAULT_WG_PORT: &str = "51820";
const DEFAULT_WG_DNS: &str = "1.1.1.1";
const DEFAULT_WG_PSK_CLIENT: &str = "00000000000000000000000000000000";
const DEFAULT_WG_PSK_PUB_SERVER: &str = "00000000000000000000000000000000";

#[allow(dead_code)]
#[derive(Debug)]
pub struct Env{
    //Wifi credentials
    pub sta_ssid: HeaplessString<32>,
    pub sta_passwd: HeaplessString<64>,

    //Wireguard credentials
    pub wg_addr: HeaplessString<16>,
    pub wg_port: HeaplessString<16>,
    pub wg_dns: HeaplessString<16>,
    pub wg_psk_client: HeaplessString<32>,
    pub wg_psk_pub_server: HeaplessString<32>,
}

impl Env{
    fn env_var(env_var: &str) -> Result<&str, anyhow::Error>{
        let var = include_str!(".env").lines()
                .find(|line| line.starts_with(&env_var))
                .unwrap_or("")
                .split('=')
                .nth(1)
                .unwrap_or("");
        
        Ok(var)
    }

    fn construct_field<const N: usize>(var: &str) -> anyhow::Result<HeaplessString<N>> {
        let var_env = Env::env_var(var)?;
        let mut var = HeaplessString::<N>::new();

        _ = var.push_str(var_env);

        Ok(var)
    }

    pub fn new() -> Result<Self, anyhow::Error>{
        Ok(Self { 
            sta_ssid: Env::construct_field::<32>("STA_SSID")
            .unwrap_or(DEFAULT_STA_SSID.try_into()?),
            
            sta_passwd: Env::construct_field::<64>("STA_PASSWD")
            .unwrap_or(DEFAULT_STA_PASSWD.try_into()?), 
            
            wg_addr: Env::construct_field::<16>("WG_ADDR")
            .unwrap_or(DEFAULT_WG_ADDR.try_into()?),
            
            wg_port: Env::construct_field::<16>("WG_PORT")
            .unwrap_or(DEFAULT_WG_PORT.try_into()?),
            
            wg_dns: Env::construct_field::<16>("WG_DNS")
            .unwrap_or(DEFAULT_WG_DNS.try_into()?),
            
            wg_psk_client: Env::construct_field::<32>("WG_PSK_CLIENT")
            .unwrap_or(DEFAULT_WG_PSK_CLIENT.try_into()?),
            
            wg_psk_pub_server: Env::construct_field::<32>("WG_PSK_PUB_SERVER")
            .unwrap_or(DEFAULT_WG_PSK_PUB_SERVER.try_into()?),

        })
    }   
}