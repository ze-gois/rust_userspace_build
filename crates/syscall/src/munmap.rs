use arch::{Arch, traits::Callable};

hooking!(MUNMAP);

use ::macros::define_error;

define_error!(
    "munmap",
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
        Err(arch::Error::TODO) => Err(crate::Error::MUnmap(Error::NotReadable)),
        Ok(no) => Ok((no, no)),
    }
}

#[inline(always)]
pub fn munmap(addr: *mut u8, length: usize) -> crate::Result {
    let arch_result = Arch::syscall2(NUMBER, addr as usize, length);
    handle_result(arch_result)
}
