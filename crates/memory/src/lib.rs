#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod page;
pub mod stack;
pub use stack::Stack;
pub mod alloc;
pub use alloc::alloc;
pub mod macros;

use arch::Pointer;
use arch::PointerType;
