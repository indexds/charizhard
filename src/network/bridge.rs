use esp_idf_svc::eth::{EspEth, RmiiEth};
use esp_idf_svc::handle::RawHandle;
use esp_idf_svc::sys::wg::wireguard_ctx_t;

use crate::wireguard::ctx::WG_CTX;

pub fn _start_bridge(eth_netif: EspEth<'static, RmiiEth>) -> anyhow::Result<()> {
    let mut lock = WG_CTX.lock().unwrap();

    let ctx: *mut wireguard_ctx_t = lock.as_mut().unwrap().get_raw();

    unsafe {
        let _wg_netif = ctx.as_ref().unwrap().netif;
        let _eth_netif = eth_netif.netif().handle();

        // create callbacks between both netifs to bridge them?
    }

    Ok(())
}
