use crate::wireguard::wireguard_ctx_t;
use std::sync::{Arc, Mutex};

pub struct WireguardCtx(*mut wireguard_ctx_t);

unsafe impl Send for WireguardCtx {}
unsafe impl Sync for WireguardCtx {}

impl WireguardCtx {
    pub fn new(ctx: *mut wireguard_ctx_t) -> Self {
        WireguardCtx(ctx)
    }

    pub fn get_raw(&self) -> *mut wireguard_ctx_t {
        self.0
    }
}

lazy_static::lazy_static!(
    pub static ref WG_CTX: Arc<Mutex<Option<WireguardCtx>>> = Arc::new(Mutex::new(None));
);
