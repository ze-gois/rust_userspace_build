#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod arch;
pub mod results;
pub mod stdout;

pub mod traits;

pub mod macros {
    pub mod traits {
        // pub use macros::traits::Bytes;
        // pub use macros::traits::Primitive;
        pub use crate::traits::Bytes;
        pub use crate::traits::Primitive;
    }

    pub use macros::r#enum;
    pub use macros::expressions_upperbound;
    pub use macros::trait_implement_bytes;
    pub use macros::trait_implement_defaut_for_primitives;
    pub use macros::trait_implement_primitive;
}

crate::macros::trait_implement_defaut_for_primitives!();

pub use results::{Error, Ok, Result};
