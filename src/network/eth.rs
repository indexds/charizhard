use core::ptr;
use esp_idf_svc::eth::{EspEth, EthDriver, RmiiClockConfig, RmiiEth, RmiiEthChipset};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::{gpio, gpio::Pins, mac::MAC};
use esp_idf_svc::ipv4::{
    ClientConfiguration as IpClientConfiguration,
    ClientSettings as IpClientSettings,
    Configuration as IpConfiguration,
    Ipv4Addr,
    Mask,
    Subnet,
};
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use esp_idf_svc::sys::esp;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

pub fn init_eth(
    pins: Pins,
    mac: MAC,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<Arc<Mutex<EspEth<'static, RmiiEth>>>> {
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

    let mut eth_netif = EspEth::wrap_all(
        eth_driver,
        EspNetif::new_with_conf(&NetifConfiguration {
            ip_configuration: Some(IpConfiguration::Client(IpClientConfiguration::Fixed(IpClientSettings {
                ip: Ipv4Addr::new(192, 168, 1, 100),
                subnet: Subnet {
                    gateway: Ipv4Addr::new(192, 168, 1, 200),
                    mask: Mask(24),
                },
                dns: Some(Ipv4Addr::new(1, 1, 1, 1)),
                secondary_dns: Some(Ipv4Addr::new(1, 0, 0, 1)),
            }))),
            stack: NetifStack::Eth,
            ..NetifConfiguration::eth_default_client()
        })?,
    )?;

    let client_mac: Arc<OnceCell<[u8; 6]>> = Arc::new(OnceCell::new());
    let client_mac2 = Arc::clone(&client_mac);

    eth_netif
        .driver_mut()
        .set_rx_callback(move |frame| match frame.as_slice().get(6..12) {
            Some(mac_bytes) => {
                let src_mac = mac_bytes.try_into().unwrap();
                if client_mac2.set(src_mac).is_ok() {
                    log::info!("Sniffed client MAC: {}", mac2str(src_mac));
                }
            }
            None => unreachable!("Failed to read source MAC from Ethernet frame!"),
        })?;

    eth_netif.start()?;

    log::info!("Waiting to sniff client MAC...");
    let _client_mac = *client_mac.wait();

    eth_netif.driver_mut().set_rx_callback(|_| {})?;

    log::warn!("Setting Ethernet promiscuous...");
    esp!(unsafe {
        use esp_idf_svc::handle::RawHandle;
        use esp_idf_svc::sys::{esp_eth_io_cmd_t_ETH_CMD_S_PROMISCUOUS, esp_eth_ioctl};
        let handle = eth_netif.driver_mut().handle();
        let mut t = true;
        esp_eth_ioctl(handle, esp_eth_io_cmd_t_ETH_CMD_S_PROMISCUOUS, ptr::addr_of_mut!(t).cast())
    })?;

    log::warn!("Ethernet promiscuous success!");

    log::info!("Starting Ethernet driver..");
    eth_netif.start()?;

    Ok(Arc::new(Mutex::new(eth_netif)))
}

#[inline]
fn mac2str(mac: [u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}
