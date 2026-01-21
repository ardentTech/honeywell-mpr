//! This example shows how to read the sensor status using Embassy.

#![no_std]
#![no_main]

use defmt::*;
use embassy_rp::i2c::InterruptHandler;
use {defmt_rtt as _, panic_probe as _};
use honeywell_mpr::{Mpr, MprConfig, TransferFunction};

embassy_rp::bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<embassy_rp::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());
    let sda = p.PIN_14;
    let scl = p.PIN_15;
    let mut config = embassy_rp::i2c::Config::default();
    config.frequency = 400_000;
    let bus = embassy_rp::i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, config);

    let config = MprConfig::new(0, 25, TransferFunction::C);
    let mut sensor = Mpr::new_i2c(bus, 0x18, config).unwrap();

    loop {
        match sensor.status().await {
            Ok(status) => info!("status: {:#x}", status.into_bits()),
            Err(_) => error!("status failed :(")
        }
        embassy_time::Timer::after(embassy_time::Duration::from_millis(3_000)).await;
    }
}