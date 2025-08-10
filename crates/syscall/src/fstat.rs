use arch::{Arch, traits::Callable};

hooking!(FSTAT);

use ::macros::define_error;

define_error!(
    "fstat",
    [[
        NotReadable,
        13,
        "File not open for reading",
        "EACCES",
        EACCES
    ]]
);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::FStat(Error::NotReadable)),
        Ok(no) => Ok((no, no)),
    }
}

#[inline(always)]
pub fn fstat(fd: isize, stat: *const u8) -> crate::Result {
    let arch_result = Arch::syscall2(NUMBER, fd as usize, stat as usize);
    handle_result(arch_result)
}
