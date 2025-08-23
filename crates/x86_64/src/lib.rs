#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod result;
pub mod syscall;

pub use result::*;
pub use syscall::*;

pub mod traits;

pub mod macros {
    pub mod traits {
        pub use crate::traits::Bytes;
        pub use crate::traits::Primitive;
    }

    pub use macros::r#enum;
    pub use macros::expressions_upperbound;
    pub use macros::r#struct;
    pub use macros::trait_implement_bytes;
    pub use macros::trait_implement_defaut_for_primitives_by_crate;
    pub use macros::trait_implement_primitive;
}

crate::macros::trait_implement_defaut_for_primitives_by_crate!(CrateX86_64);

pub use result::{Error, Ok, Result};
