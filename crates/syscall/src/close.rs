use arch::{Arch, traits::Callable};

hooking!(CLOSE);

use ::macros::define_error;

define_error!("close", []);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::Close(Error::TODO)),
        Ok(no) => Ok((no, no)),
    }
}

#[inline(always)]
pub fn close(fd: i32) -> crate::Result {
    let arch_result = Arch::syscall1(NUMBER, fd as usize);
    handle_result(arch_result)
}
