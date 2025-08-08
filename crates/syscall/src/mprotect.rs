pub mod flags;

use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::MProtect as usize;

use result::define_error;

define_error!(
    "mprotect",
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
        Err(arch::Error::TODO) => Err(crate::Error::MProtect(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

#[inline(always)]
pub fn mprotect(addr: *mut u8, len: usize, prot: i32) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, addr as usize, len, prot as usize);
    handle_result(arch_result)
}
