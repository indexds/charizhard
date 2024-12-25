use std::sync::{Arc, Mutex};

use esp_idf_svc::eth::{EspEth, EthDriver, RmiiClockConfig, RmiiEth, RmiiEthChipset};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio;
use esp_idf_svc::hal::gpio::Pins;
use esp_idf_svc::hal::mac::MAC;
use esp_idf_svc::ipv4::{
    Configuration as IpConfiguration,
    Ipv4Addr,
    Mask,
    RouterConfiguration as IpRouterConfiguration,
    Subnet,
};
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use once_cell::sync::OnceCell;

pub fn init_driver(pins: Pins, mac: MAC, sysloop: EspSystemEventLoop) -> anyhow::Result<EthDriver<'static, RmiiEth>> {
    let mut eth_driver = EthDriver::new_rmii(
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

    let client_mac: Arc<OnceCell<[u8; 6]>> = Arc::new(OnceCell::new());
    let client_mac2 = Arc::clone(&client_mac);

    eth_driver.set_rx_callback(move |frame| match frame.as_slice().get(6..12) {
        Some(mac_bytes) => {
            let src_mac = mac_bytes.try_into().unwrap();
            if client_mac2.set(src_mac).is_ok() {
                log::info!("Sniffed client MAC: {}", mac2str(src_mac));
            }
        }
        None => unreachable!("Failed to read source MAC from Ethernet frame!"),
    })?;

    eth_driver.start()?;

    log::info!("Waiting to sniff client MAC...");
    let _client_mac = *client_mac.wait();

    // stops the driver
    eth_driver.set_rx_callback(|_| {})?;

    eth_driver.set_promiscuous(true)?;

    Ok(eth_driver)
}

pub fn install_netif(eth_driver: EthDriver<'static, RmiiEth>) -> anyhow::Result<Arc<Mutex<EspEth<'static, RmiiEth>>>> {
    log::warn!("Installing eth netif...");

    let mut eth_netif = EspEth::wrap_all(
        eth_driver,
        EspNetif::new_with_conf(&NetifConfiguration {
            ip_configuration: Some(IpConfiguration::Router(IpRouterConfiguration {
                subnet: Subnet {
                    gateway: Ipv4Addr::new(192, 168, 1, 1),
                    mask: Mask(30),
                },
                dhcp_enabled: true,
                dns: None,
                secondary_dns: None,
            })),
            stack: NetifStack::Eth,
            ..NetifConfiguration::eth_default_router()
        })?,
    )?;

    eth_netif.start()?;

    log::warn!("Eth netif install success!");

    Ok(Arc::new(Mutex::new(eth_netif)))
}

#[inline]
fn mac2str(mac: [u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}