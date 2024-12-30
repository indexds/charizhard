use core::num::NonZeroU32;

use esp_idf_svc::eth::{EspEth, EthDriver, RmiiClockConfig, RmiiEth, RmiiEthChipset};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::gpio::{self, Pins};
use esp_idf_svc::hal::mac::MAC;
use esp_idf_svc::ipv4::{ClientConfiguration, ClientSettings, Configuration, Ipv4Addr, Mask, Subnet};
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use esp_idf_svc::sys::{
    esp_netif_flags_ESP_NETIF_DHCP_SERVER,
    esp_netif_flags_ESP_NETIF_FLAG_AUTOUP,
    ip_event_t_IP_EVENT_ETH_GOT_IP,
    ip_event_t_IP_EVENT_ETH_LOST_IP,
};

/// Initializes the Ethernet driver and network interface, then starts it.
pub fn start(pins: Pins, mac: MAC, sysloop: EspSystemEventLoop) -> anyhow::Result<EspEth<'static, RmiiEth>> {
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
            flags: esp_netif_flags_ESP_NETIF_DHCP_SERVER | esp_netif_flags_ESP_NETIF_FLAG_AUTOUP,
            got_ip_event_id: NonZeroU32::new(ip_event_t_IP_EVENT_ETH_GOT_IP as _),
            lost_ip_event_id: NonZeroU32::new(ip_event_t_IP_EVENT_ETH_LOST_IP as _),
            key: "ETH_DEF".try_into().unwrap(),
            description: "eth".try_into().unwrap(),
            route_priority: 50, // Higher is better
            ip_configuration: Some(Configuration::Client(ClientConfiguration::Fixed(ClientSettings {
                ip: Ipv4Addr::new(10, 10, 10, 1),
                subnet: Subnet {
                    gateway: Ipv4Addr::new(10, 10, 10, 1), // This is the gateway advertised by the dhcp server
                    mask: Mask(30),
                },
                dns: None,
                secondary_dns: None,
            }))),
            stack: NetifStack::Eth,
            custom_mac: None,
        })?,
    )?;

    log::info!("Enabling napt on eth netif..");

    // Necessary for routing packets between subnets.
    eth_netif.netif_mut().enable_napt(true);

    log::info!("Starting eth netif..");

    eth_netif.start()?;

    Ok(eth_netif)
}
