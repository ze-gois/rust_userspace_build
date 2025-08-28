#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub struct Origin;

pub use human::info;

pub mod file;
pub mod memory;

pub mod types;

macros::trait_implement_primitives!();
