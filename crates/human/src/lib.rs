#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

pub mod arch;
pub mod result;
pub mod stdout;

pub use result::*;

pub mod traits;

pub mod macros {
    pub mod traits {
        // pub use macros::traits::Bytes;
        // pub use macros::traits::Primitive;
        pub use crate::traits::Bytes;
        pub use crate::traits::Primitive;
    }
    pub mod results {
        pub mod traits {
            pub use macros::results::traits::Result;
        }
    }
    pub use macros::r#enum;
    pub use macros::result;
    pub use macros::trait_implement_bytes;
    pub use macros::trait_implement_defaut_for_primitives;
    pub use macros::trait_implement_primitive;
}

crate::macros::trait_implement_defaut_for_primitives!();

// pub mod traits;
