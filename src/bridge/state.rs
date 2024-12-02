use esp_idf_svc::eth::{EthDriver, RmiiEth};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::{modem::Modem, peripherals::Peripherals};
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::wifi::WifiDriver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub const ETH2WIFI_TASK_STACK_SIZE: usize = 2048;
pub const ETH2WIFI_TASK_PRIORITY: u8 = 19;

pub const WIFI2ETH_TASK_STACK_SIZE: usize = 2048;
pub const WIFI2ETH_TASK_PRIORITY: u8 = 19;

pub struct Bridge<S> {
    pub state: S,
}

pub struct Idle {
    pub peripherals: Peripherals,
    pub sysloop: EspSystemEventLoop,
    pub nvs: Option<EspDefaultNvsPartition>,
}

pub struct EthReady {
    pub modem: Modem,
    pub sysloop: EspSystemEventLoop,
    pub nvs: Option<EspDefaultNvsPartition>,
    pub eth: EthDriver<'static, RmiiEth>,
    pub client_mac: [u8; 6],
}

pub struct WifiReady {
    pub eth: EthDriver<'static, RmiiEth>,
    pub wifi: Arc<Mutex<WifiDriver<'static>>>,
    pub nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
}

pub struct Running {
    pub _eth2wifi_handle: JoinHandle<!>,
    pub _wifi2eth_handle: JoinHandle<!>,
}
