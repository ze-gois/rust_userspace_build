#![no_std]
#[macro_use]
pub mod enums;

#[macro_use]
pub mod result;

#[macro_use]
pub mod structs;

// #[macro_use]
// pub mod tuples;

#[macro_use]
pub mod latent;

#[macro_use]
pub mod traits;

pub mod macros {
    pub use crate::enums;
    pub use crate::latent;
    pub use crate::result;
    pub use crate::structs;
    // pub use crate::tuples;

    pub mod traits {
        pub use crate::traits::Primitive;
        pub use crate::traits::XElfSize;
    }
}

trait_implement_default_primitive!(
    bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
