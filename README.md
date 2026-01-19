# Honeywell MPR
`#![no_std]`, `async`-first driver for the Honeywell MPR pressure sensor built on top of
Rust [embedded-hal](https://github.com/rust-embedded/embedded-hal).

Chip variants (see datasheet Figure 4) differ between I2C and SPI as implementations of the serial interface.

## I2C
* Supports 100-400 kbit/s bus speeds
* 7-bit device address (see datasheet Figure 4 Output Type)

## SPI
> [!IMPORTANT]
> The SPI implementation and examples have NOT been verified or tested on hardware. If you have a chip variation that supports SPI and want to contribute, please document your setup in relevant examples, update code as needed and open a PR.

## Examples
E.g. `$ cd examples/rp235x && cargo run --bin i2c_async_status`

### Resources
* [Datasheet](https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/board-mount-pressure-sensors/micropressure-mpr-series/documents/sps-siot-mpr-series-datasheet-32332628-ciid-172626.pdf?download=false)

### License
* [MIT](https://github.com/ardentTech/honeywell-mpr/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/honeywell-mpr/blob/main/LICENSE-APACHE)