use embedded_hal_async::i2c::I2c;
use embedded_hal_async::spi::SpiDevice;
use crate::typedefs::MprError;

mod private {
    pub trait Sealed {}
}

pub trait Interface: private::Sealed {
    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), MprError>;
    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), MprError>;
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
    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), MprError> {
        self.device.read(self.address, buf).await.map_err(|_| MprError::I2c)
    }

    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), MprError> {
        self.device.write(self.address, buf).await.map_err(|_|  MprError::I2c)
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
    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), MprError> {
        self.device.read(buf).await.map_err(|_| MprError::I2c)
    }

    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), MprError> {
        self.device.write(buf).await.map_err(|_|  MprError::I2c)
    }
}