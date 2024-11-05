use crate::bridge::state::*;
use crate::utils::nvs::{NvsKeys, NvsWifi};
use core::ptr;
use esp_idf_svc::eth::{EthDriver, RmiiClockConfig, RmiiEthChipset};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::nvs::EspNvs;
use esp_idf_svc::sys::esp;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, WifiDeviceId, WifiDriver};
use once_cell::sync::OnceCell;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// <https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/performance/speed.html#task-priorities>
const ETH_TASK_PRIORITY: u8 = 19;
const ETH_TASK_STACK_SIZE: usize = 512;

const WIFI_TASK_PRIORITY: u8 = 19;
const WIFI_TASK_STACK_SIZE: usize = 512;

#[allow(dead_code)]
impl Bridge<Idle> {
    pub fn new() -> anyhow::Result<Self> {
        let peripherals = Peripherals::take()?;
        let sysloop = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take()?;

        Ok(Self {
            state: Idle {
                peripherals,
                sysloop,
                nvs,
            },
        })
    }
}

impl TryFrom<Bridge<Idle>> for Bridge<EthReady> {
    type Error = anyhow::Error;

    fn try_from(val: Bridge<Idle>) -> anyhow::Result<Self> {
        let pins = val.state.peripherals.pins;
        let mut eth = EthDriver::new_rmii(
            val.state.peripherals.mac,
            pins.gpio25, // RMII RDX0
            pins.gpio26, // RMII RDX1
            pins.gpio27, // RMII CRS DV
            pins.gpio23, // WT32-ETH01 SMI MDC
            pins.gpio22, // EMII TXD1
            pins.gpio21, // RMII TX EN
            pins.gpio19, // RMII TXD0
            pins.gpio18, // WT32-ETH01 SMI MDIO
            RmiiClockConfig::<gpio::Gpio0, gpio::Gpio16, gpio::Gpio17>::Input(
                pins.gpio0, // WT32-ETH01 external clock
            ),
            Some(pins.gpio16), // WT32-ETH01 PHY reset
            RmiiEthChipset::LAN87XX,
            Some(1), // WT32-ETH01 PHY address
            val.state.sysloop.clone(),
        )?;

        let client_mac: Arc<OnceCell<[u8; 6]>> = Arc::new(OnceCell::new());
        let client_mac2 = Arc::clone(&client_mac);

        //temporary callback to sniff the client's mac
        eth.set_rx_callback(move |frame| match frame.as_slice().get(6..12) {
            Some(mac_bytes) => {
                let src_mac = mac_bytes.try_into().expect("Failed to retrieve mac bytes.");

                if client_mac2.set(src_mac).is_ok() {
                    log::info!("Sniffed client MAC: {}", mac2str(src_mac));
                }
            }
            None => panic!("Failed to read source MAC from Ethernet frame!"),
        })?;

        log::info!("Waiting to sniff client MAC...");

        eth.start()?;
        let client_mac = *client_mac.wait();

        //reset the callback, now unneeded
        eth.set_rx_callback(|_| {})?;

        log::info!("Setting Ethernet promiscuous...");

        //black magic sets ethernet driver in promiscuous mode
        esp!(unsafe {
            use esp_idf_svc::handle::RawHandle;
            use esp_idf_svc::sys::{esp_eth_io_cmd_t_ETH_CMD_S_PROMISCUOUS, esp_eth_ioctl};
            let handle = eth.handle();
            let mut t = true;
            esp_eth_ioctl(
                handle,
                esp_eth_io_cmd_t_ETH_CMD_S_PROMISCUOUS,
                ptr::addr_of_mut!(t).cast(),
            )
        })?;

        log::info!("Ethernet promiscuous success!");

        Ok(Self {
            state: EthReady {
                modem: val.state.peripherals.modem,
                sysloop: val.state.sysloop,
                nvs: val.state.nvs,
                eth,
                client_mac,
            },
        })
    }
}

impl TryFrom<Bridge<EthReady>> for Bridge<WifiReady> {
    type Error = anyhow::Error;

    fn try_from(val: Bridge<EthReady>) -> anyhow::Result<Self> {
        let mut wifi = WifiDriver::new(
            val.state.modem,
            val.state.sysloop.clone(),
            Some(val.state.nvs.clone()),
        )?;

        wifi.set_mac(WifiDeviceId::Sta, val.state.client_mac)?;

        let nvs = EspNvs::new(val.state.nvs, "config", true)?;
        let nvs = Arc::new(Mutex::new(nvs));

        Ok(Self {
            state: WifiReady {
                eth: val.state.eth,
                wifi,
                nvs,
            },
        })
    }
}

impl TryFrom<Bridge<WifiReady>> for Bridge<Running> {
    type Error = anyhow::Error;

    fn try_from(val: Bridge<WifiReady>) -> anyhow::Result<Self> {
        let nvs = Arc::clone(&val.state.nvs);
        let nvs = nvs
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock NVS Mutex."))?;

        let ssid = NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_SSID)?
            .clean_string()
            .inner();
        let password = NvsWifi::get_field::<64>(&nvs, NvsKeys::STA_PASSWD)?
            .clean_string()
            .inner();
        let auth_method = NvsWifi::get_field::<32>(&nvs, NvsKeys::STA_AUTH_METHOD)?
            .clean_string()
            .inner();

        let auth_method = AuthMethod::from_str(auth_method.as_str())?;

        let wifi_config = Configuration::Client(ClientConfiguration {
            ssid,
            auth_method,
            password,
            ..Default::default()
        });

        let mut eth = val.state.eth;
        let mut wifi = val.state.wifi;

        wifi.set_configuration(&wifi_config)?;

        let (eth_tx, eth_rx) = mpsc::channel();
        let (wifi_tx, wifi_rx) = mpsc::channel();

        //frames received on wifi are sent to the channel
        //frames transferred on wifi are dropped
        wifi.set_callbacks(
            move |_, frame| {
                if wifi_tx.send(frame).is_err() {
                    log::warn!("Failed to send Wi-Fi frame to queue, did the receiver hangup?");
                }
                Ok(())
            },
            |_, _, _| {},
        )?;

        //frames received on ethernet are sent to the channel
        eth.set_rx_callback(move |frame| {
            if eth_tx.send(frame).is_err() {
                log::warn!("Failed to send Ethernet frame to queue, did the receiver hangup?");
            }
        })?;

        wifi.start()?;
        eth.start()?;

        ThreadSpawnConfiguration {
            name: Some(c"eth2wifi_task".to_bytes_with_nul()),
            stack_size: ETH_TASK_STACK_SIZE,
            priority: ETH_TASK_PRIORITY,
            ..Default::default()
        }
        .set()?;

        //handle every frame in the channel received from the eth
        let eth2wifi_handle = thread::spawn(move || {
            for frame in &eth_rx {
                if wifi
                    .is_connected()
                    .expect("Failed to check wifi connectivity!")
                {
                    if let Err(e) = wifi.send(WifiDeviceId::Sta, frame.as_slice()) {
                        log::error!("Failed to send frame out Wi-Fi: {}", e);
                    }
                } else {
                    log::info!("Trying to connect to Wi-Fi...");
                    if wifi.connect().is_ok() {
                        log::info!("Connected to Wi-Fi!");
                    } else {
                        log::error!("Failed to connect to Wi-Fi!");
                        log::warn!("Wi-Fi disconnected, ignoring frame.");
                    }
                }
            }
            log::warn!("Failed to consume frame from Ethernet queue! Did the sender hangup?");
        });

        ThreadSpawnConfiguration {
            name: Some(c"wifi2eth_task".to_bytes_with_nul()),
            stack_size: WIFI_TASK_STACK_SIZE,
            priority: WIFI_TASK_PRIORITY,
            ..Default::default()
        }
        .set()?;

        //handle every frame in the channel received from the wifi
        let wifi2eth_handle = thread::spawn(move || {
            for frame in &wifi_rx {
                if eth.is_connected().unwrap() {
                    if let Err(e) = eth.send(frame.as_slice()) {
                        log::error!("Failed to send frame out Ethernet: {}", e);
                    }
                } else {
                    log::warn!("Ethernet disconnected, ignoring frame.");
                }
            }
            log::warn!("Failed to consume frame from Ethernet queue! Did the sender hangup?");
        });

        Ok(Self {
            state: Running {
                eth2wifi_handle,
                wifi2eth_handle,
            },
        })
    }
}

fn mac2str(mac: [u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}
