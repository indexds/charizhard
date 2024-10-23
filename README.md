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

This project uses the [esp-idf](https://github.com/espressif/esp-idf/tree/v5.2.3) version `v5.2.3` for the standard `esp32`.

## Requirements

This project requires Rust, Python and the standard C toolchain. The standard C toolchain as well as git are assumed to already be installed by the user. If not, the user is likely on Windows and following the msys2 tutorial here should make everything work out of the box: [Install msys2](https://www.msys2.org/). 

Installing git can then be done using `pacman -Syu && pacman -S git` in a `ucrt64` shell.  
The `C:\msys64\ucrt64\bin` folder must be added to the PATH environment variable.

### Windows
**Installing Rust:**
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

**Installing Python:** ```https://www.python.org/downloads/release/python-3125/```

### Linux

**Installing Rust:**
```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

**Installing Python:** Using your favorite package manager, install `python3.12` `python3-pip` or `python312` and `python-pip`

Ensure the version is 3.12.5 as more recent versions have proved unstable.

## Installing

This project needs to be cloned in your root directory, as the esp32 cannot handle long paths.

```cd /```

Install espup to get the esp toolchain required to compile on the `xtensa-esp32-espidf` architecture. Until [*this issue*](https://github.com/esp-rs/espup/issues/440) is fixed, ensure you install the `0.11.0` version of the binary.

```cargo install espup@0.11.0```

Install the toolchain and follow any additional instruction written to the standard output.

```espup install```

Clone the repository then `cd` inside. Note that the name of the folder must be short and located at the root of your filesystem. Here, we use chhard. 

```git clone https://github.com/indexds/charizhard chhard && cd chhard```

Install the `cargo-make` binary to make the build process less of a chore. If not possible, the project *can* be compiled by running the commands found in `Makefile.toml` manually.

You're also going to need espflash and ldproxy as dependencies.

```cargo install cargo-make cargo-generate espflash ldproxy```

Then you're gonna need:

```pip install esptool```

All that remains is to install all remaining dependencies, build the project and flash the esp32 with:

```cargo flash```

## Monitoring

### Windows

On Windows, you can install Putty here: [Download Putty](https://www.putty.org/).

Then run `cargo monitor-windows` or `cargo mw`.

### Linux

On Linux, install the `screen` package using your favorite package manager, then run:

`screen /dev/ttyS* 115200` or `cargo monitor-linux` or `cargo ml`.

## Menuconfig

To access the menuconfig tool, install the following dependencies:

```cargo install cargo-pio```
```pip install --upgrade platformio```

Then, run:

```cargo pio installpio```

You can now access the menuconfig tool using:

```cargo pio espidf menuconfig```

The tool will first download the `xtensa-esp-elf` toolchain which may take a while.

## Troubleshooting

* If the compilation process fails to find `libclang.dll` or `clang.dll`, create an environment variable `LIBCLANG_PATH` with the PATH to  your `libclang.dll` as such:
### Linux
```~/.rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-clang/bin/```
### Windows
```%USERPROFILE%\.rustup\toolchains\esp\xtensa-esp32-elf-clang\esp-clang\bin\```

#

* If the program fails upon flashing the `esp32`, press and hold the `BOOT/EN` button on the SOC to put it in bootloader mode. Firmware should start flashing. 

* If the program fails because of the path length to your project, verify that you cloned the project at the root of your filesystem and that the name of the directory is as short as specified.

* If the program fails for any reason related to Python, check that the version used by the program is a `3.12.x` as Python `>=3.13` is not currently supported.

* If the program fails for any other reason, try running your shell with administrator/sudo privileges

* Should the program have multiple dependency failures, try running `cargo clean` then rebuilding the project properly with `cargo flash`.

* If you cannot move the selection while in menuconfig inside an integrated terminal like the one provided by vscode, the correct bindings are J/K.

## License

[GNU GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.fr.html#license-text)