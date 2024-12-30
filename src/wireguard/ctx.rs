use core::ptr;
use std::sync::{Arc, Mutex};

use esp_idf_svc::sys::wg::{wireguard_config_t, wireguard_ctx_t};

/// This struct wraps the raw pointers to the wireguard context. We declare it
/// Send + Sync as it needs to be passed to different threads.
pub struct Wireguard(pub *mut wireguard_ctx_t, pub *mut wireguard_config_t);

unsafe impl Send for Wireguard {}
unsafe impl Sync for Wireguard {}

impl Wireguard {
    /// This function should never be called. It only serves to initialize the
    /// [`lazy_static::lazy_static!`] macro.
    fn new(ctx: *mut wireguard_ctx_t, config: *mut wireguard_config_t) -> Self {
        Wireguard(ctx, config)
    }

    /// Stores the wireguard `ctx` and `config` context pointers for
    /// safekeeping.
    ///
    /// This function should only ever be called when a wireguard tunnel is
    /// established with a peer using [`start_tunnel`].
    ///
    /// [`start_tunnel`]: crate::wireguard::start_tunnel
    pub fn set(&mut self, ctx: *mut wireguard_ctx_t, config: *mut wireguard_config_t) {
        log::warn!("Storing Wireguard context pointers!");
        self.0 = ctx;
        self.1 = config;
    }

    /// Checks if a wireguard `ctx` context pointer is stored.
    ///
    /// If so, and unless undefined behavior is achieved by improper use of
    /// other functions we know that we are connected to a peer through a
    /// tunnel.
    pub fn is_set(&self) -> bool {
        !(self.0.is_null())
    }

    /// This function should only ever be called when a wireguard tunnel is
    /// ended with a peer using [`start_tunnel`].
    ///
    /// Care should be taken never to call this function before first calling
    /// [`esp_wireguard_disconnect`] as this would result in a memory leak,
    /// definite undefined behavior and a potential crash.
    ///
    /// [`start_tunnel`]: crate::wireguard::start_tunnel
    /// [`esp_wireguard_disconnect`]: esp_idf_svc::sys::wg::esp_wireguard_disconnect
    pub fn reset(&mut self) {
        log::warn!("Resetting Wireguard context pointers!");

        unsafe {
            let _ = Box::from_raw(self.0);
            let _ = Box::from_raw(self.1);
        }

        self.0 = ptr::null_mut();
        self.1 = ptr::null_mut();
    }
}

lazy_static::lazy_static!(
    /// This is the global hot potato that needs to never ever be dropped.
    /// Care should be taken when accessing this variable as thread safety is not guaranteed.
    pub static ref WG_CTX: Arc<Mutex<Wireguard>> = Arc::new(Mutex::new(Wireguard::new(ptr::null_mut(), ptr::null_mut())));
);
