# Honeywell MPR
`#![no_std]`, `async`-first driver for the Honeywell MPR pressure sensor built on top of
Rust [embedded-hal](https://github.com/rust-embedded/embedded-hal). If you need blocking code, simply enable the `sync` feature in your `Cargo.toml`.

## Chip Variants
See datasheet Figure 4 for a description of available chip variants. For implementing the correct driver, make note of
the "Output Type" (I2C or SPI), the I2C address (if relevant), and the transfer function.

## I2C
* Supports 100-400 kbit/s bus speeds
* 7-bit device address (see datasheet Figure 4 Output Type)
* [Async Examples](https://github.com/ardentTech/honeywell-mpr/examples/rp235x_async)
* [Blocking Examples](https://github.com/ardentTech/honeywell-mpr/examples/rp235x_blocking)

## SPI
> [!WARNING]
> The SPI implementation has NOT been verified or tested on hardware, and that is why there are no examples. If you have
> a SPI-based chip variation and want to contribute, please document your setup in relevant examples, update code as
> needed and open a PR.

### Resources
* [Datasheet](https://prod-edam.honeywell.com/content/dam/honeywell-edam/sps/siot/en-us/products/sensors/pressure-sensors/board-mount-pressure-sensors/micropressure-mpr-series/documents/sps-siot-mpr-series-datasheet-32332628-ciid-172626.pdf?download=false)

### License
* [MIT](https://github.com/ardentTech/honeywell-mpr/blob/main/LICENSE-MIT)
* [Apache](https://github.com/ardentTech/honeywell-mpr/blob/main/LICENSE-APACHE)