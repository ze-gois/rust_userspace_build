use arch::{Arch, traits::Callable};

hooking!(CLOSE);

use result::define_error;

define_error!(
    "close",
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
        Err(arch::Error::TODO) => Err(crate::Error::Close(Error::NotReadable)),
        Ok(no) => Ok((no, no)),
    }
}

#[inline(always)]
pub fn close(fd: i32) -> crate::Result {
    let arch_result = Arch::syscall1(NUMBER, fd as usize);
    handle_result(arch_result)
}
