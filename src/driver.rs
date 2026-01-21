#[cfg(not(feature = "sync"))]
use embedded_hal_async::delay::DelayNs;
#[cfg(not(feature = "sync"))]
use embedded_hal_async::i2c::I2c;
#[cfg(not(feature = "sync"))]
use embedded_hal_async::spi::SpiDevice;


#[cfg(feature = "sync")]
use embedded_hal::delay::DelayNs;
#[cfg(feature = "sync")]
use embedded_hal::i2c::I2c;
#[cfg(feature = "sync")]
use embedded_hal::spi::SpiDevice;

use crate::interface::{I2cInterface, Interface, MprI2cError, MprSpiError, SpiInterface};
use crate::{MprConfig, Reading};
use crate::registers::Status;

const EXIT_STANDBY_DELAY_MS: u32 = 10;
const OUTPUT_MEASUREMENT_CMD: [u8; 3] = [0xaa, 0x00, 0x00];
const VALID_I2C_ADDRESSES: [u8; 8] = [0x08, 0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78];

/// MPR driver instance consisting of configuration and a I2C or SPI interface.
#[derive(Debug)]
pub struct Mpr<I> {
    config: MprConfig,
    interface: I
}

/// Constructs a MPR driver instance using the I2C interface.
impl <I2C: I2c> Mpr<I2cInterface<I2C>> {
    pub fn new_i2c(device: I2C, address: u8, config: MprConfig) -> Result<Mpr<I2cInterface<I2C>>, MprI2cError<I2C::Error>>  {
        if !VALID_I2C_ADDRESSES.contains(&address) {
            return Err(MprI2cError::InvalidAddress)
        }
        Ok(Mpr { config, interface: I2cInterface::new(device, address) })
    }
}

/// Constructs a MPR driver instance using the SPI interface.
impl <SPI: SpiDevice> Mpr<SpiInterface<SPI>> {
    pub fn new_spi(device: SPI, config: MprConfig) -> Result<Mpr<SpiInterface<SPI>>, MprSpiError<SPI::Error>>  {
        Ok(Mpr { config, interface: SpiInterface::new(device) })
    }
}

impl <I: Interface>Mpr<I> {

    /// Exits sensor standby mode and enters operating mode in preparation for measurement.
    ///
    /// App should delay >=5ms or wait for rising edge on EOC line after this returns and before
    /// reading measurement data via any `read_raw*` method.
    #[maybe_async::maybe_async]
    pub async fn exit_standby(&mut self) -> Result<(), I::Error> {
        self.interface.write_reg(&OUTPUT_MEASUREMENT_CMD).await
        // TODO should this return Status (first byte?) MISO on SPI, but a dedicated read on I2C...
    }

    /// Reads 24-bits of raw pressure data.
    #[maybe_async::maybe_async]
    pub async fn read_raw(&mut self) -> Result<u32, I::Error> {
        let mut buf = [0u8; 4];
        self.interface.read_reg(&mut buf).await?;

        let status = Status::from_bits(buf[0]);
        self.interface.validate_status(status)?;
        Ok(((buf[1] as u32) << 16) + ((buf[2] as u32) << 8) + buf[3] as u32)
    }

    /// Exits standby, waits and then reads raw pressure data.
    #[maybe_async::maybe_async]
    pub async fn read_raw_with_delay<D: DelayNs>(&mut self, mut delay: D) -> Result<u32, I::Error> {
        self.exit_standby().await?;
        delay.delay_ms(EXIT_STANDBY_DELAY_MS).await;
        self.read_raw().await
    }

    /// Reads 24-bits of raw pressure data as a Reading.
    #[maybe_async::maybe_async]
    pub async fn read(&mut self) -> Result<Reading, I::Error> {
        let raw_data = self.read_raw().await?;
        Ok(Reading {
            range_min: self.config.pressure_min as f32,
            range_max: self.config.pressure_max as f32,
            raw_data,
            transfer_function: self.config.transfer_function
        })
    }

    /// Exits standby, waits and then reads raw pressure data as a Reading.
    #[maybe_async::maybe_async]
    pub async fn read_with_delay<D: DelayNs>(&mut self, mut delay: D) -> Result<Reading, I::Error> {
        self.exit_standby().await?;
        delay.delay_ms(EXIT_STANDBY_DELAY_MS).await;
        let raw_data = self.read_raw().await?;
        Ok(Reading {
            range_min: self.config.pressure_min as f32,
            range_max: self.config.pressure_max as f32,
            raw_data,
            transfer_function: self.config.transfer_function
        })
    }

    /// Reads the sensor status byte.
    #[maybe_async::maybe_async]
    pub async fn status(&mut self) -> Result<Status, I::Error> {
        let mut buf = [0u8; 1];
        self.interface.read_reg(&mut buf).await?;
        Ok(Status::from_bits(buf[0]))
    }
}