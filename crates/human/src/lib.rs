#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod arch;
// pub mod macros;
pub mod result;
pub mod stdout;
pub mod syscall;

pub use result::{Error, Ok, Result};

pub struct Origin {}

macros::trait_implement_primitives!();
