use arch::{Arch, traits::Callable};

pub mod flags;
pub use flags::Flag;

hooking!(LSEEK);

use ::macros::define_error;

define_error!(
    "lseek",
    [[BadFileDescriptor, 9, "Bad file descriptor", "EBADF", EBADF],]
);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::LSeek(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

#[inline(always)]
pub fn lseek(fd: i32, offset: i64, whence: i32) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, fd as usize, offset as usize, whence as usize);

    handle_result(arch_result)
}
