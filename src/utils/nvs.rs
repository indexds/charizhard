use crate::utils::heapless::HeaplessString;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use std::sync::MutexGuard;


const DEFAULT_STA_SSID: &str = "ssid";
const DEFAULT_STA_PASSWD: &str = "passwd";

const DEFAULT_WG_ADDR: &str = "0.0.0.0/24";
const DEFAULT_WG_PORT: &str = "51820";
const DEFAULT_WG_DNS: &str = "1.1.1.1";

const DEFAULT_WG_CLIENT_PRIV_KEY: &str = "";
const DEFAULT_WG_SERVER_PUB_KEY: &str = "";


#[derive(serde::Deserialize, Debug)]
pub struct Nvs{

    pub sta_ssid: HeaplessString<32>,
    pub sta_passwd: HeaplessString<64>,

    pub wg_addr: HeaplessString<32>,
    pub wg_port: HeaplessString<16>,
    pub wg_dns: HeaplessString<32>,

    pub wg_client_priv_key: HeaplessString<32>,
    pub wg_server_pub_key: HeaplessString<32>,
}


#[allow(dead_code)]
impl Nvs{
    pub fn get_field<const N: usize>(nvs: &MutexGuard<'_, EspNvs<NvsDefault>>, key: &str) -> anyhow::Result<HeaplessString<N>> {

        let mut buf = [0u8; N];
        nvs.get_str(key, &mut buf)?;

        let mut value = HeaplessString::<N>::new();
        value.push_str(core::str::from_utf8(&buf)?)?;

        if value.clean_string().trim()?.is_empty(){

            return Err(anyhow::anyhow!("String is empty!"))
        }
        
        Ok(value)
    }

    pub fn set_field(nvs: &mut MutexGuard<'_, EspNvs<NvsDefault>>, key: &str, value: &str) -> anyhow::Result<()> {
        
        nvs.set_str(key, value)?;

        Ok(())
    }

    pub fn new(nvs: MutexGuard<'_, EspNvs<NvsDefault>>) -> anyhow::Result<Self>{

        Ok(Self { 

            sta_ssid: Nvs::get_field::<32>(&nvs, "STA_SSID")
            .unwrap_or(DEFAULT_STA_SSID.try_into()?),
            
            sta_passwd: Nvs::get_field::<64>(&nvs, "STA_PASSWD")
            .unwrap_or(DEFAULT_STA_PASSWD.try_into()?), 
            
            wg_addr: Nvs::get_field::<32>(&nvs, "WG_ADDR")
            .unwrap_or(DEFAULT_WG_ADDR.try_into()?),
            
            wg_port: Nvs::get_field::<16>(&nvs, "WG_PORT")
            .unwrap_or(DEFAULT_WG_PORT.try_into()?),
            
            wg_dns: Nvs::get_field::<32>(&nvs, "WG_DNS")
            .unwrap_or(DEFAULT_WG_DNS.try_into()?),
            
            wg_client_priv_key: Nvs::get_field::<32>(&nvs, "WG_CLIENT_PRIV_KEY")
            .unwrap_or(DEFAULT_WG_CLIENT_PRIV_KEY.try_into()?),
            
            wg_server_pub_key: Nvs::get_field::<32>(&nvs, "WG_SERVER_PUB_KEY")
            .unwrap_or(DEFAULT_WG_SERVER_PUB_KEY.try_into()?),

        })
    }   
}