pub mod flags;

pub use flags::*;
use macros::labeled_enum_typed;

use super::Number;
use arch::{Arch, Callable};

static NUMBER: usize = Number::MMap as usize;

macros::labeled_enum_typed!(
    Error,
    isize,
    isize,
    "mmap",
    [[
        NotReadable,
        EACCES,
        -13,
        "File not open for reading",
        "EACCES"
    ]]
);

#[inline(always)]
pub fn mmap(
    addr: *mut u8,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> crate::Result<isize> {
    let arch_result = Arch::syscall6(
        NUMBER,
        addr as usize,
        length,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    );

    handle_result(arch_result)
}
