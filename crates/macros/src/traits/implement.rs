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
