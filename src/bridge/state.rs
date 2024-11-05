use esp_idf_svc::eth::{EthDriver, RmiiEth};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, NvsDefault};
use esp_idf_svc::wifi::WifiDriver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct Bridge<State> {
    pub state: State,
}

pub struct Idle {
    pub peripherals: Peripherals,
    pub sysloop: EspSystemEventLoop,
    pub nvs: EspDefaultNvsPartition,
}

pub struct EthReady {
    pub modem: Modem,
    pub sysloop: EspSystemEventLoop,
    pub nvs: EspDefaultNvsPartition,
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