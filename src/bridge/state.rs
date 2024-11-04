use esp_idf_svc::nvs::{EspNvs, NvsDefault, EspDefaultNvsPartition};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::eth::{RmiiEth, EthDriver};
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::wifi::WifiDriver;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};

pub struct Bridge<State> {
    pub state: State,
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
    pub wifi: WifiDriver<'static>,
    pub nvs: Arc<Mutex<EspNvs<NvsDefault>>>,
}

#[allow(dead_code)]
pub struct Running {
    pub eth2wifi_handle: JoinHandle<()>,
    pub wifi2eth_handle: JoinHandle<()>,
}