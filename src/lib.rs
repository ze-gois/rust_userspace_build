#![no_std]

pub mod dtype;
pub mod panic;
pub mod result;

pub use self::result::{Error, Result};
pub use human::info;
pub use macros;
