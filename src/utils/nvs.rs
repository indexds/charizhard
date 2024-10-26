use crate::utils::heapless::HeaplessString;
use esp_idf_svc::nvs::{EspDefaultNvs, EspDefaultNvsPartition, EspNvs, NvsDefault};

const DEFAULT_STA_SSID: &str = "";
const DEFAULT_STA_PASSWD: &str = "";
const DEFAULT_WG_ADDR: &str = "0.0.0.0/24";
const DEFAULT_WG_PORT: &str = "51820";
const DEFAULT_WG_DNS: &str = "1.1.1.1";
const DEFAULT_WG_PSK_CLIENT: &str = "default_psk";
const DEFAULT_WG_PSK_PUB_SERVER: &str = "default_psk_pub";

#[allow(dead_code)]
pub struct Nvs{

    //Wifi credentials
    pub sta_ssid: HeaplessString<32>,
    pub sta_passwd: HeaplessString<64>,

    //Wireguard credentials
    pub wg_addr: HeaplessString<32>,
    pub wg_port: HeaplessString<16>,
    pub wg_dns: HeaplessString<32>,
    pub wg_psk_client: HeaplessString<32>,
    pub wg_psk_pub_server: HeaplessString<32>,
}

#[allow(dead_code)]
impl Nvs{
    pub fn get_field<const N: usize>(nvs: &EspNvs<NvsDefault>, key: &str) -> anyhow::Result<HeaplessString<N>> {

        let mut buf = [0u8; N];
        nvs.get_str(key, &mut buf)?;

        let mut value = HeaplessString::<N>::new();
        value.push_str(core::str::from_utf8(&buf)?)?;

        Ok(value)
    }

    pub fn set_field(nvs: &mut EspNvs<NvsDefault>, key: &str, value: &str) -> anyhow::Result<()> {
        
        nvs.set_str(key, value)?;

        Ok(())
    }

    pub fn new() -> Result<Self, anyhow::Error>{

        let nvs = EspDefaultNvs::new(EspDefaultNvsPartition::take()?, "config", false)?;

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
            
            wg_psk_client: Nvs::get_field::<32>(&nvs, "WG_PSK_CLIENT")
            .unwrap_or(DEFAULT_WG_PSK_CLIENT.try_into()?),
            
            wg_psk_pub_server: Nvs::get_field::<32>(&nvs, "WG_PSK_PUB_SERVER")
            .unwrap_or(DEFAULT_WG_PSK_PUB_SERVER.try_into()?),

        })
    }   
}