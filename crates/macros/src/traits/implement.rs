pub mod bytes;
pub mod from;
pub mod partial_eq;
pub mod primitive;

#[macro_export]
macro_rules! trait_implement_primitives {
    ($($t:ty),*) => {
        macros::trait_implement_primitive!(
            true, bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128,
            usize
        );
        macros::trait_implement_bytes!(
            bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
        );

        impl macros::traits::Bytes<crate::Origin, crate::Origin> for () {
            const BYTES_SIZE: usize = core::mem::size_of::<()>();
            fn from_bytes(_bytes: [u8; <Self as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE], _endianness: bool) -> Self {
                ()
            }
            fn to_bytes(&self, _endianness: bool) -> [u8; <Self as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
                [0u8; 0]
            }
        }
    };
}

// #[macro_export]
// macro_rules! trait_implement_defaut_for_primitives_by_crate {
//     ($crate_id:ident) => {
//         pub trait $crate_id {}

//         macros::trait_implement_primitive!(
//             $crate_id, true, bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32,
//             u64, u128, usize
//         );

//         macros::trait_implement_bytes!(
//             $crate_id, bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64,
//             u128, usize
//         );
//     };
// }
