#![no_std]
#![allow(async_fn_in_trait)]

use embedded_hal_async::i2c::I2c;
use crate::registers::Status;
use crate::typedefs::TransferFunction;

mod registers;
pub mod typedefs;

const EXIT_STANDBY_DELAY_MS: u32 = 10;
const OUTPUT_MEASUREMENT_CMD: [u8; 3] = [0xaa, 0x00, 0x00];
const VALID_I2C_ADDRESSES: [u8; 8] = [0x08, 0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78];


// COMMON ------------------------------------------------------------------------------------------
pub struct MprConfig {
    pressure_min: u8,
    pressure_max: u8,
    transfer_function: TransferFunction
}
impl MprConfig {
    pub fn new(
        pressure_min: u8,
        pressure_max: u8,
        transfer_function: TransferFunction
    ) -> Self {
        Self { pressure_min, pressure_max, transfer_function }
    }
}

#[derive(Debug)]
pub enum MprError {
    I2c,
    InvalidAddress,
    Nop
}

// TODO seal this
pub trait Interface {
    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), MprError>;
    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), MprError>;
}

// I2C ---------------------------------------------------------------------------------------------
pub struct I2cInterface<I2C> {
    address: u8,
    device: I2C
}

impl<I2C: I2c> I2cInterface<I2C> {
    pub fn new(device: I2C, address: u8) -> Self {
        Self { device, address }
    }
}

impl<I2C: I2c>Interface for I2cInterface<I2C> {
    async fn read_reg(&mut self, buf: &mut [u8]) -> Result<(), MprError> {
        self.device.read(self.address, buf).await.map_err(|_| MprError::I2c)
    }

    async fn write_reg(&mut self, buf: &[u8]) -> Result<(), MprError> {
        self.device.write(self.address, buf).await.map_err(|_|  MprError::I2c)
    }
}

// SPI ---------------------------------------------------------------------------------------------

// DRIVER ------------------------------------------------------------------------------------------

pub struct Mpr<I> {
    config: MprConfig,
    interface: I
}

impl <I2C: I2c> Mpr<I2cInterface<I2C>> {
    pub fn new_i2c(device: I2C, address: u8, config: MprConfig) -> Result<Mpr<I2cInterface<I2C>>, MprError>  {
        if !VALID_I2C_ADDRESSES.contains(&address) {
            return Err(MprError::InvalidAddress)
        }
        Ok(Mpr { config, interface: I2cInterface::new(device, address) })
    }
}
// TODO spi

impl <I: Interface>Mpr<I> {
    pub async fn status(&mut self) -> Result<Status, MprError> {
        let mut buf = [0u8; 1];
        self.interface.read_reg(&mut buf).await?;
        Ok(Status::from_bits(buf[0]))
    }
}