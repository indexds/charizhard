use core::ptr;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

use esp_idf_svc::eth::{EspEth, RmiiEth};
use esp_idf_svc::handle::RawHandle;
use esp_idf_svc::netif::EspNetif;
use esp_idf_svc::sys::{
    _g_esp_netif_netstack_default_br,
    bridgeif_config,
    esp,
    esp_netif_attach,
    esp_netif_br_glue_add_port,
    esp_netif_br_glue_new,
    esp_netif_config_t,
    esp_netif_flags_ESP_NETIF_FLAG_IS_BRIDGE,
    esp_netif_inherent_config,
    esp_netif_new,
};

fn start(eth: Arc<Mutex<EspEth<'static, RmiiEth>>>, wg: Arc<Mutex<EspNetif>>) -> anyhow::Result<()> {
    let bridge_info = Box::new(bridgeif_config {
        max_fdb_dyn_entries: 10,
        max_fdb_sta_entries: 10,
        max_ports: 2,
    });

    let base = Box::new(esp_netif_inherent_config {
        flags: esp_netif_flags_ESP_NETIF_FLAG_IS_BRIDGE,
        mac: [0x02, 0x00, 0x00, 0x00, 0x00, 0x10],
        ip_info: ptr::null_mut(),
        get_ip_event: 0,
        lost_ip_event: 0,
        if_key: CString::new("br0")?.into_raw(),
        if_desc: CString::new("bridge")?.into_raw(),
        route_prio: 30,
        bridge_info: Box::into_raw(bridge_info),
    });

    let netif_conf = esp_netif_config_t {
        base: Box::into_raw(base),
        driver: ptr::null_mut(),
        stack: unsafe { _g_esp_netif_netstack_default_br },
    };

    let bridge_netif = unsafe { esp_netif_new(&netif_conf) };

    let glue = unsafe { esp_netif_br_glue_new() };

    let eth_handle = eth.lock().unwrap().netif_mut().handle();
    let wg_handle = wg.lock().unwrap().handle();

    esp!(unsafe { esp_netif_br_glue_add_port(glue, eth_handle) })?;
    esp!(unsafe { esp_netif_br_glue_add_port(glue, wg_handle) })?;

    esp!(unsafe { esp_netif_attach(bridge_netif, glue as _) })?;

    Ok(())
}
