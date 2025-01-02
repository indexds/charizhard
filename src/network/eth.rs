use std::sync::{Arc, Mutex};

use esp_idf_svc::eth::{EspEth, EthDriver, RmiiClockConfig, RmiiEth, RmiiEthChipset};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio::{self, Pins};
use esp_idf_svc::hal::mac::MAC;
use esp_idf_svc::ipv4::{
    Configuration,
    Ipv4Addr,
    Mask,
    RouterConfiguration,
    Subnet,
};
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use once_cell::sync::OnceCell;

/// Initializes the Ethernet driver and network interface, then starts it.
pub fn start(
    pins: Pins,
    mac: MAC,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<Arc<Mutex<EspEth<'static, RmiiEth>>>> {
    log::info!("Initializing eth driver..");

    let eth_driver = EthDriver::new_rmii(
        mac,
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
        sysloop,
    )?;

    log::info!("Installing ethernet netif...");

    let mut eth_netif = EspEth::wrap_all(
        eth_driver,
        EspNetif::new_with_conf(&NetifConfiguration {
            flags: 0,
            key: "ETH_DEF".try_into().unwrap(),
            description: "eth".try_into().unwrap(),
            route_priority: 10,
            ip_configuration: Some(Configuration::Router(RouterConfiguration {
                subnet: Subnet {
                    gateway: Ipv4Addr::new(10, 10, 10, 1),
                    mask: Mask(30),
                },
                dhcp_enabled: true, //adds dhcp_server flag
                dns: None,
                secondary_dns: None,
            })),
            stack: NetifStack::Eth,
            custom_mac: None,
            got_ip_event_id: None,
            lost_ip_event_id: None,
        })?,
    )?;

    log::info!("Starting ethernet netif...");
    eth_netif.start()?;

    let client_mac: Arc<OnceCell<[u8; 6]>> = Arc::new(OnceCell::new());
    
    let mac_ref = Arc::clone(&client_mac);

    eth_netif.driver_mut().set_rx_callback(move |frame| match frame.as_slice().get(6..12) {
        Some(mac_bytes) => {
            let src_mac = mac_bytes.try_into().unwrap();
            if mac_ref.set(src_mac).is_ok() {
                log::warn!("Sniffed client MAC: {}", mac2str(src_mac));
            }
        }
        None => unreachable!("Failed to read source MAC from ethernet frame!"),
    })?;

    log::warn!("Waiting to sniff client MAC...");
    eth_netif.start()?;

    let client_mac = *client_mac.wait();

    eth_netif.driver_mut().set_rx_callback(|_| {})?;

    log::info!("Setting ethernet netif mac to client mac..");

    eth_netif.netif_mut().set_mac(&client_mac)?;

    log::warn!("Setting Ethernet promiscuous...");
    eth_netif.driver_mut().set_promiscuous(true)?;

    Ok(Arc::new(Mutex::new(eth_netif)))
}

/// Format MAC bytes as a hex string.
///
/// E.g. `02:aa:bb:cc:12:34`
#[inline]
fn mac2str(mac: [u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}