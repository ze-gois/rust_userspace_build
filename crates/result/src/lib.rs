#![no_std]

#[macro_use]
pub mod macros;

/// A trait that standardizes error handling across the xelf crates
///
/// This trait allows for consistent error handling patterns by providing:
/// - Error creation from errno values
/// - Error descriptions
/// - Optional advertising of error codes for syscall returns
pub trait ErrorTrait<T> {
    /// Creates an error instance from a numeric error code
    fn from_no(errno: T) -> Self;

    fn to_no(&self) -> T;

    /// Returns a human-readable description of the error
    fn description(&self) -> &str;

    fn acronym(&self) -> &str;
}

pub trait ErrorNestedTrait<T, U> {
    fn from_no(errno_holder: T, errno_held: U) -> Self;

    fn to_no(&self) -> (T, U);

    fn description(&self) -> &str;

    fn acronym(&self) -> &str;
}
