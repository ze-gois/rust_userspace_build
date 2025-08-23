#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod macros;
pub mod result;
pub mod syscall;

pub use result::{Error, Ok, Result};
pub use syscall::*;

pub mod page {
    pub const SIZE: usize = 0x1000;
}
