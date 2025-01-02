use core::ptr;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

use esp_idf_svc::eth::{EspEth, RmiiEth};
use esp_idf_svc::handle::RawHandle;
use esp_idf_svc::sys::{
    _g_esp_netif_netstack_default_br,
    bridgeif_config_t,
    esp,
    esp_netif_attach,
    esp_netif_br_glue_add_port,
    esp_netif_br_glue_add_wifi_port,
    esp_netif_br_glue_new,
    esp_netif_bridge_add_port,
    esp_netif_config_t,
    esp_netif_flags_ESP_NETIF_FLAG_IS_BRIDGE,
    esp_netif_inherent_config,
    esp_netif_new,
    esp_netif_t,
};
use esp_idf_svc::wifi::EspWifi;

// use crate::wireguard::ctx::WG_CTX;

fn create_config() -> anyhow::Result<*mut esp_netif_config_t> {
    let bridge_info = Box::new(bridgeif_config_t {
        max_fdb_dyn_entries: 1,
        max_fdb_sta_entries: 1,
        max_ports: 2,
    });

    let inherent_config = Box::new(esp_netif_inherent_config {
        flags: esp_netif_flags_ESP_NETIF_FLAG_IS_BRIDGE,
        mac: [0x02, 0x00, 0x00, 0x00, 0x00, 0x01], // Needs to be unique, first 0x02 = LAA
        ip_info: ptr::null(),
        get_ip_event: 0,
        lost_ip_event: 0,
        if_key: CString::new("br0")?.into_raw(),
        if_desc: CString::new("bridge")?.into_raw(),
        route_prio: 30,
        bridge_info: Box::into_raw(bridge_info),
    });

    let bridge_config = Box::new(esp_netif_config_t {
        base: Box::into_raw(inherent_config),
        driver: ptr::null_mut(),
        stack: unsafe { _g_esp_netif_netstack_default_br },
    });

    Ok(Box::into_raw(bridge_config))
}

pub fn start(
    eth_netif: Arc<Mutex<EspEth<'static, RmiiEth>>>,
    wifi_netif: Arc<Mutex<EspWifi<'static>>>,
) -> anyhow::Result<*mut esp_netif_t> {
    unsafe {
        let bridge_netif = esp_netif_new(create_config()?);
        let bridge_glue = esp_netif_br_glue_new();

        if bridge_glue.is_null() {
            panic!("glue is null!");
        }

        let eth_handle = eth_netif.lock().unwrap().netif_mut().handle();
        let wifi_handle = wifi_netif.lock().unwrap().sta_netif_mut().handle();

        esp!(esp_netif_br_glue_add_port(bridge_glue, eth_handle))?;
        esp!(esp_netif_br_glue_add_port(bridge_glue, wifi_handle))?;

        esp!(esp_netif_attach(bridge_netif, bridge_glue as _))?;

        Ok(bridge_netif)
    }
}
