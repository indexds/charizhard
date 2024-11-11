use crate::utils::heapless::HeaplessString;
use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use serde::Deserialize;
use std::sync::MutexGuard;

const DEFAULT_STA_SSID: &str = "";
const DEFAULT_STA_PASSWD: &str = "";
const DEFAULT_STA_AUTH_METHOD: &str = "wpa2personal";

const DEFAULT_WG_ADDR: &str = "";
const DEFAULT_WG_PORT: &str = "";

const DEFAULT_WG_CLIENT_PRIV_KEY: &str = "";
const DEFAULT_WG_SERVER_PUB_KEY: &str = "";

pub struct NvsKeys;

impl NvsKeys {
    pub const STA_AUTH_METHOD: &'static str = "AUTH";
    pub const STA_PASSWD: &'static str = "PASSWD";
    pub const STA_SSID: &'static str = "SSID";
    pub const WG_ADDR: &'static str = "ADDR";
    pub const WG_CLIENT_PRIV_KEY: &'static str = "PRIVKEY";
    pub const WG_PORT: &'static str = "PORT";
    pub const WG_SERVER_PUB_KEY: &'static str = "PUBKEY";
}

#[derive(serde::Deserialize, Debug)]
pub struct NvsWireguard {
    #[serde(rename = "address")]
    pub wg_addr: HeaplessString<32>,

    #[serde(rename = "port")]
    pub wg_port: HeaplessString<16>,

    #[serde(rename = "privkey")]
    pub wg_client_priv_key: HeaplessString<44>,

    #[serde(rename = "pubkey")]
    pub wg_server_pub_key: HeaplessString<44>,
}

impl NvsWireguard {
    pub fn get_field<const N: usize>(
        nvs: &MutexGuard<'_, EspNvs<NvsDefault>>,
        key: &str,
    ) -> anyhow::Result<HeaplessString<N>> {
        let mut buf = [0u8; N];
        nvs.get_str(key, &mut buf)?;

        let raw_value = core::str::from_utf8(&buf)
            .map(|s| s.trim_end_matches(char::from('0')))
            .unwrap_or("");

        let mut value = HeaplessString::<N>::new();
        value.push_str(&raw_value)?;

        if value.clean_string().trim()?.is_empty() {
            return Err(anyhow::anyhow!("String is empty!"));
        }

        Ok(value)
    }

    pub fn set_field(nvs: &mut MutexGuard<'_, EspNvs<NvsDefault>>, key: &str, value: &str) -> anyhow::Result<()> {
        nvs.set_str(key, value.trim())?;

        Ok(())
    }

    pub fn new(nvs: &MutexGuard<'_, EspNvs<NvsDefault>>) -> anyhow::Result<Self> {
        Ok(Self {
            wg_addr: NvsWireguard::get_field::<32>(&nvs, NvsKeys::WG_ADDR).unwrap_or(DEFAULT_WG_ADDR.try_into()?),

            wg_port: NvsWireguard::get_field::<16>(&nvs, NvsKeys::WG_PORT).unwrap_or(DEFAULT_WG_PORT.try_into()?),

            wg_client_priv_key: NvsWireguard::get_field::<44>(&nvs, NvsKeys::WG_CLIENT_PRIV_KEY)
                .unwrap_or(DEFAULT_WG_CLIENT_PRIV_KEY.try_into()?),

            wg_server_pub_key: NvsWireguard::get_field::<44>(&nvs, NvsKeys::WG_SERVER_PUB_KEY)
                .unwrap_or(DEFAULT_WG_SERVER_PUB_KEY.try_into()?),
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct NvsWifi {
    #[serde(rename = "ssid")]
    pub sta_ssid: HeaplessString<32>,

    #[serde(rename = "passwd")]
    pub sta_passwd: HeaplessString<64>,

    #[serde(rename = "authmethod")]
    pub sta_auth_method: HeaplessString<32>,
}

#[allow(dead_code)]
impl NvsWifi {
    pub fn get_field<const N: usize>(
        nvs: &MutexGuard<'_, EspNvs<NvsDefault>>,
        key: &str,
    ) -> anyhow::Result<HeaplessString<N>> {
        let mut buf = [0u8; N];
        nvs.get_str(key, &mut buf)?;

        let raw_value = core::str::from_utf8(&buf)
            .map(|s| s.trim_end_matches(char::from('0')))
            .unwrap_or("");

        let mut value = HeaplessString::<N>::new();
        value.push_str(&raw_value)?;

        Ok(value)
    }

    pub fn set_field(nvs: &mut MutexGuard<'_, EspNvs<NvsDefault>>, key: &str, value: &str) -> anyhow::Result<()> {
        nvs.set_str(key, value.trim())?;

        Ok(())
    }

    pub fn new(nvs: &MutexGuard<'_, EspNvs<NvsDefault>>) -> anyhow::Result<Self> {
        Ok(Self {
            sta_ssid: NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_SSID).unwrap_or(DEFAULT_STA_SSID.try_into()?),

            sta_passwd: NvsWifi::get_field::<64>(&nvs, NvsKeys::STA_PASSWD).unwrap_or(DEFAULT_STA_PASSWD.try_into()?),

            sta_auth_method: NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_AUTH_METHOD)
                .unwrap_or(DEFAULT_STA_AUTH_METHOD.try_into()?),
        })
    }
}
