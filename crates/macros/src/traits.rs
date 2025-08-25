pub mod bytes;
pub mod enums;
pub mod primitive;

#[macro_use]
pub mod implement;

// crate::trait_place_bytes!();
pub use bytes::Bytes;
pub use primitive::Primitive;
// crate::trait_place_primitive!();
