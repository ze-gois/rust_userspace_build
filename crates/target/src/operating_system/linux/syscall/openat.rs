use crate::arch::{Arch, traits::Callable};

pub use crate::operating_system::syscall::open::flags;
pub use crate::operating_system::syscall::open::{Error, Ok, Result};

hooking!(OPENAT);

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

pub fn handle_result(result: crate::Result) -> crate::Result {
    // Err(crate::Error::Default(1))
    match result {
        crate::Result::Ok(crate::Ok::Arch(crate::arch::Ok::X86_64Syscall(
            crate::arch::syscall::Ok::X86_64Syscall3(crate::arch::syscall::syscall3::Ok::Default(
                m,
            )),
        ))) => core::result::Result::Ok(crate::Ok::Os(crate::os::Ok::Syscall(
            crate::os::syscall::Ok::Open(crate::os::syscall::open::Ok::OPENAT(m)),
        ))),
        _ => core::result::Result::Err(crate::Error::Os(crate::os::Error::Syscall(
            crate::os::syscall::Error::Open(Error::Default(3)),
        ))),
    }
}
