use arch::{Arch, traits::Callable};

pub use crate::open::flags;
pub use crate::open::{Error, Result};

static NUMBER: usize = super::numbers::OPENAT4;

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::Open(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

pub fn openat4(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result {
    let syscall_result = Arch::syscall4(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );
    handle_result(syscall_result)
}
