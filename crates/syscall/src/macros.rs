// #[macro_use]
// pub mod handle_result;
// #[macro_use]
// pub mod syscall_error;
#[macro_use]
pub mod publishing;
#[macro_use]
pub mod hooking;

pub use macros::traits;

macros::trait_implement_primitives!();
// macros::trait_implement_primitives!();

// pub mod traits {
//     macros::trait_place_bytes!(1);
//     macros::trait_place_primitive!();
// }

// pub use macros::r#enum;
// pub use macros::expressions_upperbound;
// pub use macros::r#struct;
// pub use macros::trait_implement_bytes;
// pub use macros::trait_implement_defaut_for_primitives;
// pub use macros::trait_implement_primitive;

// macros::trait_implement_defaut_for_primitives!();
