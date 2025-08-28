use crate::arch::{Arch, traits::Callable};

pub use super::open::flags;
pub use super::open::{Error, Ok, Result};

hooking!(OPENAT4);

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

pub fn handle_result(result: crate::Result) -> crate::Result {
    // Err(crate::Error::Default(1))
    match result {
        crate::Result::Ok(_ok) => match _ok {
            // crate::Ok::Arch(crate::arch::Ok::X86_64Syscall(crate::arch::syscall::Ok::X86_64Syscall1(crate::arch::syscall::syscall1::Ok::Default(no)))) => core::result::Result::Ok(crate::Ok::Os(crate::os::Ok::Syscall()
            _ => core::result::Result::Err(crate::Error::Info(2)),
        },
        crate::Result::Err(_) => core::result::Result::Err(crate::Error::Os(
            crate::os::Error::Syscall(crate::os::syscall::Error::Open(Error::OPENAT4(3))),
        )),
    }
}
