use crate::{Arch, Callable};

use ::macros::define_error;

define_error!("munmap", []);

pub fn handle_result(arch_result: crate::Result) -> super::Result {
    match arch_result {
        Err(crate::Error::TODO) => Err(super::Error::MUnMap(Error::TODO)),
        Ok(no) => Ok((no, no)),
    }
}

#[inline(always)]
pub fn munmap(addr: *mut u8, length: usize) -> super::Result {
    let arch_result = Arch::syscall2(13, addr as usize, length);
    handle_result(arch_result)
}
