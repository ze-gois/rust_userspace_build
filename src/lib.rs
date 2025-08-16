#![no_std]

pub mod dtype;
pub mod header;
pub mod panic;
pub mod result;
pub use result::{Error, Result};

pub use human::info;
