use core::ptr::null_mut;
use std::sync::{Arc, Mutex};

use esp_idf_svc::sys::wg::wireguard_ctx_t;

/// This struct wraps the raw pointer to the wireguard context. We declare it
/// Send + Sync as it needs to be passed to different threads.
pub struct WireguardCtx(pub *mut wireguard_ctx_t);

unsafe impl Send for WireguardCtx {}
unsafe impl Sync for WireguardCtx {}

impl WireguardCtx {
    /// This function should never be called. It only serves to initialize the
    /// [`lazy_static::lazy_static!`] macro.
    fn new(ctx: *mut wireguard_ctx_t) -> Self {
        WireguardCtx(ctx)
    }

    /// Stores the wireguard `ctx` context pointer for safekeeping.
    ///
    /// This function should only ever be called when a wireguard tunnel is
    /// established with a peer using [`crate::wireguard::mod::start_tunnel`].
    pub fn set(&mut self, ctx: *mut wireguard_ctx_t) {
        log::warn!("Storing Wireguard context pointer!");
        self.0 = ctx;
    }

    /// Checks if a wireguard `ctx` context pointer is stored.
    ///
    /// If so, and unless undefined behavior is achieved by improper use of
    /// other functions we know that we are connected to a peer through a
    /// tunnel.
    pub fn is_set(&self) -> bool {
        !(self.0.is_null())
    }

    /// Dereferences the wireguard `ctx` context pointer from safekeeping.
    ///
    /// This function should only ever be called when a wireguard tunnel is
    /// ended with a peer using [`crate::wireguard::mod::start_tunnel`].
    ///
    /// Care should be taken never to call this function before first calling
    /// [`esp_wireguard_disconnect`] as this would result in a memory leak,
    /// definite undefined behavior and a potential crash.
    pub fn reset(&mut self) {
        log::warn!("Resetting Wireguard context pointer!");
        self.0 = null_mut();
    }
}

lazy_static::lazy_static!(
    /// This is the global hot potato that needs to never ever be dropped.
    /// Care should be taken when accessing this variable as thread safety is not guaranteed.
    pub static ref WG_CTX: Arc<Mutex<WireguardCtx>> = Arc::new(Mutex::new(WireguardCtx::new(null_mut())));
);
