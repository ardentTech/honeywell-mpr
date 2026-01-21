#[cfg(not(feature = "sync"))]
use embedded_hal_async::i2c::I2c;
#[cfg(not(feature = "sync"))]
use embedded_hal_async::spi::SpiDevice;
#[cfg(feature = "sync")]
use embedded_hal::i2c::I2c;
#[cfg(feature = "sync")]
use embedded_hal::spi::SpiDevice;
use crate::registers::Status;

#[derive(Debug)]
pub enum MprI2cError<E> {
    I2c(E),
    InvalidAddress,
    IntegrityTest,
    MathSaturation,
}

#[derive(Debug)]
pub enum MprSpiError<E> {
    IntegrityTest,
    MathSaturation,
    Spi(E),
}

mod private {
    pub trait Sealed {}
}

#[maybe_async::maybe_async(AFIT)]
pub trait Interface: private::Sealed {
    type Error;

    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;

    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
    fn validate_status(&self, status: Status) -> Result<(), Self::Error>;
}

// I2C ---------------------------------------------------------------------------------------------

pub struct I2cInterface<I2C> {
    address: u8,
    device: I2C
}

impl<I2C: I2c> I2cInterface<I2C> {
    pub(crate) fn new(device: I2C, address: u8) -> Self {
        Self { device, address }
    }
}
impl<I2C: I2c>private::Sealed for I2cInterface<I2C> {}
impl<I2C: I2c>Interface for I2cInterface<I2C> {
    type Error = MprI2cError<I2C::Error>;

    #[maybe_async::maybe_async]
    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), MprI2cError<I2C::Error>> {
        self.device.read(self.address, buf).await.map_err(MprI2cError::I2c)
    }

    #[maybe_async::maybe_async]
    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), MprI2cError<I2C::Error>> {
        self.device.write(self.address, buf).await.map_err(MprI2cError::I2c)
    }

    fn validate_status(&self, status: Status) -> Result<(), MprI2cError<I2C::Error>> {
        if status.math_saturation_occurred() {
            return Err(MprI2cError::MathSaturation)
        }
        if !status.integrity_test_passed() {
            return Err(MprI2cError::IntegrityTest)
        }
        Ok(())
    }
}

// SPI ---------------------------------------------------------------------------------------------

pub struct SpiInterface<SPI> {
    device: SPI
}
impl<SPI: SpiDevice> SpiInterface<SPI> {
    pub(crate) fn new(device: SPI) -> Self {
        Self { device }
    }
}
impl<SPI: SpiDevice>private::Sealed for SpiInterface<SPI> {}
impl<SPI: SpiDevice>Interface for SpiInterface<SPI> {
    type Error = MprSpiError<SPI::Error>;

    #[maybe_async::maybe_async]
    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), MprSpiError<SPI::Error>> {
        self.device.read(buf).await.map_err(MprSpiError::Spi)
    }

    #[maybe_async::maybe_async]
    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), MprSpiError<SPI::Error>> {
        self.device.write(buf).await.map_err(MprSpiError::Spi)
    }

    fn validate_status(&self, status: Status) -> Result<(), MprSpiError<SPI::Error>> {
        if status.math_saturation_occurred() {
            return Err(MprSpiError::MathSaturation)
        }
        if !status.integrity_test_passed() {
            return Err(MprSpiError::IntegrityTest)
        }
        Ok(())
    }
}