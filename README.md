# `fanshim-rs`, Fan SHIM driver

A fast, usable and safe userland driver for the Pimoroni Fan SHIM Fan and LED.

## Features

* _Very_ low CPU and memory usage.
* Easy configuration.
* Easy installation.
* Broad compatibility.

## Limitations

* No button support.
* No advanced LED support.

## Usage

Install the driver as described below.

After installation the driver will automatically be enabled.
Configuration can then be performed by opening and editing the `/etc/fanshim-rs.toml` file.
After performing changes you will either need to restart your machine, or write `sudo systemctl restart fanshim-rs` in order for the changes to take effect.

The full configuration file is:
```toml
[general]
    # Milliseconds between temperature checks.
    # 1000 would be one second.
    milliseconds_delay_between_readings = 2500

    # Amount of temperature checks before the fan enables/disables.
    number_of_readings_before_action = 3

[fan]
    # Temperature in celsius to enable the fan.
    enable_fan_at_degrees_celsius = 55

    # Temperature in celsius to disable the fan.
    disable_fan_at_degrees_celsius = 45

[led]
    # Value from 0.0 to 1.0. Set to 0.0 to disable LED.
    # Notice that 1.0 is _very_ bright. At or below 0.1 should suffice.
    led_brightness = 0.1

    [led.fan_status]
        # Color of the LED when the fan is on. 
        # RGB format, where [ 255, 0, 0] is maximum red.
        led_on_color = [ 255, 0, 0 ]

        # Color of the LED when the fan is off.
        # RGB format, where [ 0, 255, 0] is maximum green.
        led_off_color = [ 0, 255, 0 ]
```
If the configuration file is not found the defaults above will be used.

In order to see the logfiles, use `sudo journalctl -u fanshim-rs`.

The manual file can be accessed at any time by typing `man fanshim-rs`.

## Installation

`.deb` packages are provided.
These have been tested on Raspbian/Raspberry Pi OS.
The binaries are statically linked using `musl` and do not have any external dependencies.

### Raspbian/Raspberry Pi OS

#### Installation

Download the `.deb` file from the releases section.
Run `sudo dpkg -i fanshim-rs_*.deb` to install the driver.
The driver will automatically be enabled, if you're happy with the default config you won't have to do anything else.
If you want to disable the LED you will have to edit the config file as described above.

#### Removal

Run `sudo apt purge fanshim-rs`.
This will also remove your config files.

### Non-Debian based OS

A built binary is included in the releases section.
The binary can be used standalone with the default settings, or you can manually create a config file at `/etc/fanshim-rs.toml`.

## Building from source

You will need the latest stable version of Rust, found [here](https://www.rust-lang.org/tools/install).

If you've downloaded everything on your Raspberry Pi, you should just type `cargo build --release`.
The binary will be in the `target` directory.
If you're crossbuilding from your x86 computer to your Raspberry Pi, you will need to use `cross`:
```bash
cargo install cross
cargo build --target armv7-unknown-linux-gnueabihf
```

## See Also

* [The official software library (Python)](https://github.com/pimoroni/fanshim-python)
* [A C version (flobernd)](https://github.com/flobernd/raspi-fanshim)
* [A C++ Version (daviehh)](https://github.com/daviehh/fanshim-cpp)
* [Issue that spawned the non-Python versions](https://github.com/pimoroni/fanshim-python/issues/19)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
