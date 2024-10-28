use crate::utils::heapless::HeaplessString;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use std::sync::MutexGuard;


const DEFAULT_STA_SSID: &str = "";
const DEFAULT_STA_PASSWD: &str = "";

const DEFAULT_WG_ADDR: &str = "0.0.0.0/24";
const DEFAULT_WG_PORT: &str = "51820";
const DEFAULT_WG_DNS: &str = "1.1.1.1";

const DEFAULT_WG_CLIENT_PRIV_KEY: &str = "";
const DEFAULT_WG_SERVER_PUB_KEY: &str = "";

pub struct NvsKeys;

impl NvsKeys{
    pub const STA_SSID: &'static str = "SSID";
    pub const STA_PASSWD: &'static str = "PASSWD";

    pub const WG_ADDR: &'static str = "ADDR";
    pub const WG_PORT: &'static str = "PORT";
    pub const WG_DNS: &'static str = "DNS";

    pub const WG_CLIENT_PRIV_KEY: &'static str = "PRIVKEY";
    pub const WG_SERVER_PUB_KEY: &'static str = "PUBKEY";
}

#[derive(serde::Deserialize, Debug)]
pub struct Nvs{

    #[serde(rename = "ssid")]
    pub sta_ssid: HeaplessString<32>,
    
    #[serde(rename = "passwd")]
    pub sta_passwd: HeaplessString<64>,

    #[serde(rename = "address")]
    pub wg_addr: HeaplessString<32>,
    
    #[serde(rename = "port")]
    pub wg_port: HeaplessString<16>,
    
    #[serde(rename = "dns")]
    pub wg_dns: HeaplessString<32>,

    #[serde(rename = "privkey")]
    pub wg_client_priv_key: HeaplessString<32>,
    
    #[serde(rename = "pubkey")]
    pub wg_server_pub_key: HeaplessString<32>,
}


#[allow(dead_code)]
impl Nvs{
    pub fn get_field<const N: usize>(nvs: &MutexGuard<'_, EspNvs<NvsDefault>>, key: &str) -> anyhow::Result<HeaplessString<N>> {

        let mut buf = [0u8; N];
        nvs.get_str(key, &mut buf)?;

        let raw_value = core::str::from_utf8(&buf).map(|s| s.trim_end_matches(char::from('0'))).unwrap_or("");

        let mut value = HeaplessString::<N>::new();
        value.push_str(&raw_value)?;

        if value.clean_string().trim()?.is_empty(){

            return Err(anyhow::anyhow!("String is empty!"))
        }


        
        Ok(value)
    }

    pub fn set_field(nvs: &mut MutexGuard<'_, EspNvs<NvsDefault>>, key: &str, value: &str) -> anyhow::Result<()> {
        
        nvs.set_str(key, value.trim())?;

        Ok(())
    }

    pub fn new(nvs: MutexGuard<'_, EspNvs<NvsDefault>>) -> anyhow::Result<Self>{

        Ok(Self { 

            sta_ssid: Nvs::get_field::<32>(&nvs, NvsKeys::STA_SSID)
            .unwrap_or(DEFAULT_STA_SSID.try_into()?),
            
            sta_passwd: Nvs::get_field::<64>(&nvs, NvsKeys::STA_PASSWD)
            .unwrap_or(DEFAULT_STA_PASSWD.try_into()?), 
            
            wg_addr: Nvs::get_field::<32>(&nvs, NvsKeys::WG_ADDR)
            .unwrap_or(DEFAULT_WG_ADDR.try_into()?),
            
            wg_port: Nvs::get_field::<16>(&nvs, NvsKeys::WG_PORT)
            .unwrap_or(DEFAULT_WG_PORT.try_into()?),
            
            wg_dns: Nvs::get_field::<32>(&nvs, NvsKeys::WG_DNS)
            .unwrap_or(DEFAULT_WG_DNS.try_into()?),
            
            wg_client_priv_key: Nvs::get_field::<32>(&nvs, NvsKeys::WG_CLIENT_PRIV_KEY)
            .unwrap_or(DEFAULT_WG_CLIENT_PRIV_KEY.try_into()?),
            
            wg_server_pub_key: Nvs::get_field::<32>(&nvs, NvsKeys::WG_SERVER_PUB_KEY)
            .unwrap_or(DEFAULT_WG_SERVER_PUB_KEY.try_into()?),

        })
    }   
}