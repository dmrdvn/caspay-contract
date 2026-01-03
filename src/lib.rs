#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod caspay;
pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

pub use caspay::CasPay;
