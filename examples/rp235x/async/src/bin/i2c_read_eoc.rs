//! This example shows how to read sensor data using Embassy. Instead of waiting, it uses the EOC
//! pin to check for a pulse to know when data is ready.

#![no_std]
#![no_main]

use defmt::*;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::i2c::InterruptHandler;
use {defmt_rtt as _, panic_probe as _};
use honeywell_mpr::{Mpr, MprConfig, TransferFunction};

embassy_rp::bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<embassy_rp::peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_task_spawner: embassy_executor::Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut eoc = Input::new(p.PIN_13, Pull::Down);
    let sda = p.PIN_14;
    let scl = p.PIN_15;
    let mut config = embassy_rp::i2c::Config::default();
    config.frequency = 400_000;
    let bus = embassy_rp::i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, config);

    let config = MprConfig::new(0, 25, TransferFunction::C);
    let mut sensor = Mpr::new_i2c(bus, 0x18, config).unwrap();

    loop {
        sensor.exit_standby().await.unwrap();
        eoc.wait_for_high().await;
        match sensor.read().await {
            Ok(reading) => {
                info!(
                    "bar: {}, inHg: {}, mmHg: {}, kPa: {}, psi: {}",
                    reading.bar(),
                    reading.inhg(),
                    reading.mmhg(),
                    reading.kpa(),
                    reading.psi()
                );
            },
            Err(_) => error!("read failed :(")
        }
        embassy_time::Timer::after(embassy_time::Duration::from_millis(3_000)).await;
    }
}