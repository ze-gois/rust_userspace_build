#![no_std]
#![allow(unused)]

#[macro_use]
pub mod human;
pub mod r#macro;
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

pub mod macros {
    pub mod traits {
        // pub use macros::traits::Bytes;
        // pub use macros::traits::Primitive;
        pub use macros::traits::Bytes;
        pub use macros::traits::Primitive;
    }

    pub use macros::r#enum;
    pub use macros::expressions_upperbound;
    pub use macros::r#struct;
    pub use macros::trait_implement_bytes;
    pub use macros::trait_implement_defaut_for_primitives_by_crate;
    pub use macros::trait_implement_primitive;
}

crate::macros::trait_implement_defaut_for_primitives_by_crate!(ArchCrate);

pub use result::{Error, Ok, Result};
