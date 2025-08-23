#![no_std]

pub mod dtype;
pub mod header;
pub mod macros;
pub mod panic;
pub mod result;
pub use result::{Error, Ok, Result};

pub use human::info;
