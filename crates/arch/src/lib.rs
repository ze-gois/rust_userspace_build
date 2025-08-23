#![no_std]
#![allow(unused)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

#[macro_use]
pub mod human;
pub mod macros;
pub mod memory;
pub mod traits;

#[cfg(target_arch = "x86_64")]
pub use x86_64 as arch;
#[cfg(target_arch = "x86_64")]
pub mod _x86_64;
#[cfg(target_arch = "x86_64")]
pub use _x86_64 as _arch;

pub use _arch::*;
pub use arch::*;

pub use traits::*;

pub struct Arch;

pub use result::{Error, Ok, Result};
