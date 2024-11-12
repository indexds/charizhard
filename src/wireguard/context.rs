use esp_idf_svc::sys::wireguard::wireguard_ctx_t;

#[allow(dead_code)]
pub struct WireguardContext {
    ctx_ptr: *mut wireguard_ctx_t,
}

unsafe impl Send for WireguardContext {}
unsafe impl Sync for WireguardContext {}

#[allow(dead_code)]
impl WireguardContext {
    pub fn new(ctx_ptr: *mut wireguard_ctx_t) -> Self {
        Self {
            ctx_ptr,
        }
    }

    pub fn get_ptr(&self) -> *mut wireguard_ctx_t {
        self.ctx_ptr
    }
}
