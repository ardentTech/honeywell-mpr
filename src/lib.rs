#![no_std]
#![allow(async_fn_in_trait)]

pub use crate::typedefs::{MprConfig, Reading, TransferFunction};
pub use crate::driver::Mpr;

mod registers;
mod typedefs;
mod interface;
mod driver;