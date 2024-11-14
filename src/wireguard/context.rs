use esp_idf_svc::sys::wireguard::wireguard_ctx_t;
use std::sync::Mutex;

pub static mut WG_CTX: Mutex<Option<WgCtx>> = Mutex::new(None);

pub struct WgCtx {
    pub ctx_ptr: *mut wireguard_ctx_t,
}

unsafe impl Send for WgCtx {}
unsafe impl Sync for WgCtx {}

#[allow(dead_code)]
impl WgCtx {
    pub fn new(ctx_ptr: *mut wireguard_ctx_t) -> Self {
        Self {
            ctx_ptr,
        }
    }

    pub fn get_ptr(&self) -> *mut wireguard_ctx_t {
        self.ctx_ptr
    }
}
