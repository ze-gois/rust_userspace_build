#[macro_use]
pub mod default;

#[macro_use]
pub mod error;

/// A trait that standardizes error handling across the xelf crates
///
/// This trait allows for consistent error handling patterns by providing:
/// - Error creation from errno values
/// - Error descriptions
/// - Optional advertising of error codes for syscall returns
pub trait ErrorTrait {
    /// Creates an error instance from a numeric error code
    fn from_no(errno: usize) -> Self;

    fn to_no(&self) -> usize;

    /// Returns a human-readable description of the error
    fn description(&self) -> &str;

    fn acronym(&self) -> &str;

    fn from_ptr(ptr: *const u8) -> Self;

    fn as_ptr(ptr: Self) -> *const u8;
}

#[macro_use]
pub mod error_nested;

pub trait ErrorNestedTrait {
    fn from_no(errno_holder: usize, errno_held: usize) -> Self;

    fn to_no(&self) -> (usize, usize);

    fn description(&self) -> &str;

    fn acronym(&self) -> &str;

    fn from_ptr(ptr: *const u8) -> Self;

    fn as_ptr(&self) -> *const u8;
}

#[macro_use]
pub mod error_typed;
