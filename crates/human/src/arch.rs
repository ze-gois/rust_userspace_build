#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64 as arch;

pub use arch::results::{Error, Ok};

pub struct Arch;

pub fn write(file_descriptor: isize, byte_buffer: *const u8, byte_count: usize) -> crate::Result {
    let returned_value = arch::syscall3(
        1usize,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(returned_value)
}
