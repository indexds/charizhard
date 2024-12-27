use std::sync::{Arc, Mutex, MutexGuard};

use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use heapless::String;
use serde::Deserialize;

use crate::utils::heapless::HeaplessString;

#[derive(Deserialize)]
pub struct NvsWireguard {
    #[serde(rename = "address")]
    pub address: HeaplessString<32>,

    #[serde(rename = "port")]
    pub port: HeaplessString<16>,

    #[serde(rename = "privkey")]
    pub client_private_key: HeaplessString<64>,

    #[serde(rename = "pubkey")]
    pub server_public_key: HeaplessString<64>,
}

impl NvsWireguard {
    const ADDR: &'static str = "ADDR";
    const CLIENT_PRIV: &'static str = "PRIVKEY";
    const DEFAULT_ADDR: &str = "";
    const DEFAULT_CLIENT_PRIV: &str = "";
    const DEFAULT_PORT: &str = "51820";
    const DEFAULT_SERVER_PUB: &str = "";
    const PORT: &'static str = "PORT";
    const SERVER_PUB: &'static str = "PUBKEY";

    fn get_field<const N: usize>(nvs: &MutexGuard<'_, EspNvs<NvsDefault>>, key: &str) -> anyhow::Result<String<N>> {
        let mut buf = [0u8; N];
        nvs.get_str(key, &mut buf)?;

        let raw_value = core::str::from_utf8(&buf)
            .map(|s| s.trim_end_matches('0'))
            .unwrap_or("");

        let mut value = HeaplessString::<N>::new();
        value.push_str(raw_value)?;

        Ok(value.clean_string().0)
    }

    /// Call to set the Wireguard configuration in nvs.
    pub fn set_fields(nvs: Arc<Mutex<EspNvs<NvsDefault>>>, keys: NvsWireguard) -> anyhow::Result<()> {
        let mut nvs = nvs.lock().unwrap();

        nvs.set_str(Self::ADDR, keys.address.clean_string().as_str())?;
        nvs.set_str(Self::PORT, keys.port.clean_string().as_str())?;
        nvs.set_str(Self::CLIENT_PRIV, keys.client_private_key.clean_string().as_str())?;
        nvs.set_str(Self::SERVER_PUB, keys.server_public_key.clean_string().as_str())?;

        Ok(())
    }

    /// Call to get an instance of NvsWireguard containing the current stored
    /// Wireguard configs.
    pub fn new(nvs: Arc<Mutex<EspNvs<NvsDefault>>>) -> anyhow::Result<Self> {
        let nvs = nvs.lock().unwrap();

        Ok(Self {
            address: HeaplessString(
                NvsWireguard::get_field::<32>(&nvs, Self::ADDR)
                    .unwrap_or_else(|_| Self::DEFAULT_ADDR.try_into().unwrap()),
            ),

            port: HeaplessString(
                NvsWireguard::get_field::<16>(&nvs, Self::PORT)
                    .unwrap_or_else(|_| Self::DEFAULT_PORT.try_into().unwrap()),
            ),

            client_private_key: HeaplessString(
                NvsWireguard::get_field::<64>(&nvs, Self::CLIENT_PRIV)
                    .unwrap_or_else(|_| Self::DEFAULT_CLIENT_PRIV.try_into().unwrap()),
            ),

            server_public_key: HeaplessString(
                NvsWireguard::get_field::<64>(&nvs, Self::SERVER_PUB)
                    .unwrap_or_else(|_| Self::DEFAULT_SERVER_PUB.try_into().unwrap()),
            ),
        })
    }
}

#[derive(Deserialize)]
pub struct NvsWifi {
    #[serde(rename = "ssid")]
    pub sta_ssid: HeaplessString<32>,

    #[serde(rename = "passwd")]
    pub sta_passwd: HeaplessString<64>,

    #[serde(rename = "authmethod")]
    pub sta_auth: HeaplessString<32>,
}

impl NvsWifi {
    const DEFAULT_STA_AUTH: &str = "wpa2personal";
    const DEFAULT_STA_PASSWD: &str = "";
    const DEFAULT_STA_SSID: &str = "";
    const STA_AUTH: &'static str = "AUTH";
    const STA_PASSWD: &'static str = "PASSWD";
    const STA_SSID: &'static str = "SSID";

    fn get_field<const N: usize>(nvs: &MutexGuard<'_, EspNvs<NvsDefault>>, key: &str) -> anyhow::Result<String<N>> {
        let mut buf = [0u8; N];
        nvs.get_str(key, &mut buf)?;

        let raw_value = core::str::from_utf8(&buf)
            .map(|s| s.trim_end_matches('0'))
            .unwrap_or("");

        let mut value = HeaplessString::<N>::new();
        value.push_str(raw_value)?;

        Ok(value.clean_string().0)
    }

    /// Call to set the wifi configuration in nvs.
    pub fn set_fields(nvs: Arc<Mutex<EspNvs<NvsDefault>>>, keys: NvsWifi) -> anyhow::Result<()> {
        let mut nvs = nvs.lock().unwrap();

        nvs.set_str(Self::STA_SSID, keys.sta_ssid.clean_string().as_str())?;
        nvs.set_str(Self::STA_PASSWD, keys.sta_passwd.clean_string().as_str())?;
        nvs.set_str(Self::STA_AUTH, keys.sta_auth.clean_string().as_str())?;

        Ok(())
    }

    /// Call to get an instance of NvsWifi containing the current stored wifi
    /// configs.
    pub fn new(nvs: Arc<Mutex<EspNvs<NvsDefault>>>) -> anyhow::Result<Self> {
        let nvs = nvs.lock().unwrap();

        // These cannot fail, so we don't care about the unwraps
        Ok(Self {
            sta_ssid: HeaplessString(
                NvsWifi::get_field::<32>(&nvs, Self::STA_SSID)
                    .unwrap_or_else(|_| Self::DEFAULT_STA_SSID.try_into().unwrap()),
            ),

            sta_passwd: HeaplessString(
                NvsWifi::get_field::<64>(&nvs, Self::STA_PASSWD)
                    .unwrap_or_else(|_| Self::DEFAULT_STA_PASSWD.try_into().unwrap()),
            ),

            sta_auth: HeaplessString(
                NvsWifi::get_field::<32>(&nvs, Self::STA_AUTH)
                    .unwrap_or_else(|_| Self::DEFAULT_STA_AUTH.try_into().unwrap()),
            ),
        })
    }
}
