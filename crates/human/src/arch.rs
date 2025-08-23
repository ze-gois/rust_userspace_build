#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64 as arch;

pub use arch::syscall::*;
pub use arch::{Error, Ok, Result};

pub struct Arch;
