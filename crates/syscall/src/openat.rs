use arch::{Arch, traits::Callable};

pub use crate::open::flags;
pub use crate::open::{Error, Ok, Result};

hooking!(OPENAT);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::Error(no)) => {
            core::result::Result::Err(crate::Error::Open(Error::OPENAT(no)))
        }
        Ok(arch::Ok::Ok(no)) => core::result::Result::Ok(crate::Ok::Open(Ok::OPENAT(no))),
    }
}

pub fn openat(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
    );

    handle_result(syscall_result)
}
