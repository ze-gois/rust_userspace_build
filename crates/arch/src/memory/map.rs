pub mod flags;
pub mod mmap;
pub mod munmap;
pub mod prot;
pub mod result;

pub use flags::Flag;
pub use mmap::mmap;
pub use munmap::munmap;
pub use prot::Prot;

pub use result::{Error, Ok, Result};
