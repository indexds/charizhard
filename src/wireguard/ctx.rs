use std::sync::{Arc, Mutex};

use esp_idf_svc::sys::wg::wireguard_ctx_t;

pub struct WireguardCtx(pub *mut wireguard_ctx_t);

unsafe impl Send for WireguardCtx {}
unsafe impl Sync for WireguardCtx {}

impl WireguardCtx {
    pub fn new(ctx: *mut wireguard_ctx_t) -> Self {
        WireguardCtx(ctx)
    }
}

// Global hot potato that needs to never ever be dropped
lazy_static::lazy_static!(
    pub static ref WG_CTX: Arc<Mutex<Option<WireguardCtx>>> = Arc::new(Mutex::new(None));
);
