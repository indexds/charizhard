use esp_idf_svc::eth::{EspEth, EthDriver, RmiiClockConfig, RmiiEth, RmiiEthChipset};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio::{self, Pins};
use esp_idf_svc::hal::mac::MAC;
use esp_idf_svc::ipv4::{
    Configuration as IpConfiguration,
    Ipv4Addr,
    Mask,
    RouterConfiguration as IpRouterConfiguration,
    Subnet,
};
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};

pub fn init_netif(pins: Pins, mac: MAC, sysloop: EspSystemEventLoop) -> anyhow::Result<EspEth<'static, RmiiEth>> {
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

    log::info!("Installing eth netif...");

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
            flags: 0,
            ..NetifConfiguration::eth_default_router()
        })?,
    )?;

    log::info!("Starting eth netif..");

    eth_netif.start()?;

    Ok(eth_netif)
}
