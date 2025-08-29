#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub struct Origin;

pub use macros;
pub use target;

pub mod file;
pub mod license;
pub mod memory;
pub mod panic;
pub mod result;
pub mod types;
pub use result::{Error, Ok, Result};

pub use target::info;

macros::trait_implement_primitives!();
