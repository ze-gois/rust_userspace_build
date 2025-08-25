#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod dtype;
pub mod header;
// pub mod macros;
pub mod panic;
pub mod result;
pub use result::{Error, Ok, Result};

pub use human::info;

pub struct Origin;

macros::trait_implement_primitives!();
