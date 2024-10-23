<div align="center">

<img src="logo.png" width="25%" />

# CharizHard

A cutting-edge hardware solution designed to secure data exchanges and protect devices from hardware-based threats. 
</div>

## Features

This USB dongle features dual interfaces:

• **USB Interface:** The laptop detects the dongle as an Ethernet device, ensuring that the laptop’s internal network hardware
remains isolated and protected.

• **Wi-Fi Baseband Interface:** This interface connects the laptop to a Wi-Fi network, while the dongle transparently manages a
VPN connection to a secure VPN server, ensuring that all communications are encrypted and secure.

The primary goals of CharizHard are to:

• **Secure communications:** Encrypt and protect all data exchanged between the laptop and the VPN server, typically managed
by the user’s company.

• **Prevent hardware attacks:** Shield the laptop’s hardware network interface, reducing exposure to physical attacks and
vulnerabilities.

With CharizHard, users can trust that their data and devices are secure, providing peace of mind in an increasingly connected
world.


## Installing

This project needs to be cloned in your root directory, as the esp32 cannot handle long paths.

```cd /```

```cargo install espup@0.11.0```

```espup install```

```git clone https://github.com/indexds/charizhard chhard && cd chhard```

```cargo install cargo-make```

```cargo flash```

## Implementation details

.

## License

[GNU GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.fr.html#license-text)