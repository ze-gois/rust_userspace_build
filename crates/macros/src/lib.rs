#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_const_items)]

#[macro_use]
pub mod latent;

#[macro_use]
pub mod results;

#[macro_use]
pub mod enums;

#[macro_use]
pub mod structs;

#[macro_use]
pub mod traits;

#[macro_use]
pub mod tuples;

pub mod macros {
    pub use crate::*;
    //     // pub use crate::enums;
    //     // pub use crate::latent;
    //     // pub use crate::results;
    //     // pub use crate::structs;
    //     // pub use crate::traits;
    //     // pub use crate::tuples;
}
