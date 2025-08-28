#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]
pub struct Origin;

pub mod architecture;
pub mod target;
// pub mod macros;
pub mod result;
pub mod stdout;

pub use result::{Error, Ok, Result};

macros::trait_implement_primitives!();
