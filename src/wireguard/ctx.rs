use core::ptr::null_mut;
use std::sync::{Arc, Mutex};

use esp_idf_svc::sys::wg::wireguard_ctx_t;

pub struct WireguardCtx(pub *mut wireguard_ctx_t);

unsafe impl Send for WireguardCtx {}
unsafe impl Sync for WireguardCtx {}

impl WireguardCtx {
    pub fn new(ctx: *mut wireguard_ctx_t) -> Self {
        WireguardCtx(ctx)
    }

    pub fn set(&mut self, ctx: *mut wireguard_ctx_t) {
        self.0 = ctx;
    }

    pub fn is_set(&self) -> bool {
        !(self.0.is_null())
    }

    pub fn reset(&mut self) {
        self.0 = null_mut();
    }
}

// Global hot potato that needs to never ever be dropped
lazy_static::lazy_static!(
    pub static ref WG_CTX: Arc<Mutex<WireguardCtx>> = Arc::new(Mutex::new(WireguardCtx::new(null_mut())));
);
