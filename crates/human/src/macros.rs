pub mod traits {
    macros::trait_place_bytes!(1);
    macros::trait_place_primitive!();
}

pub use macros::r#enum;
pub use macros::expressions_upperbound;
pub use macros::r#struct;
pub use macros::trait_implement_bytes;
pub use macros::trait_implement_defaut_for_primitives;
pub use macros::trait_implement_primitive;

macros::trait_implement_defaut_for_primitives!();
