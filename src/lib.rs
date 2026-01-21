#![no_std]
#![allow(async_fn_in_trait)]

pub use crate::driver::Mpr;
pub use crate::interface::{MprI2cError, MprSpiError};
pub use crate::typedefs::{MprConfig, Reading, TransferFunction};

mod registers;
mod typedefs;
mod interface;
mod driver;