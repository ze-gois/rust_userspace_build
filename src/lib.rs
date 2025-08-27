#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub struct Origin;

pub mod dtype;
pub mod header;
pub mod license;
pub mod panic;
pub mod result;
pub use result::{Error, Ok, Result};

pub use human::info;

macros::trait_implement_primitives!();
