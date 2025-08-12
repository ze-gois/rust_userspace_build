use crate::{Arch, Callable};

::macros::define_error!("mmap", []);

pub fn handle_result(arch_result: crate::Result) -> super::Result {
    match arch_result {
        Err(crate::Error::TODO) => Err(super::Error::MMap(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

#[inline(always)]
#[rustfmt::skip]
pub fn mmap(addr: *mut u8, length: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> super::Result {
    let arch_result = Arch::syscall6(
        9,
        addr as usize,
        length,
        prot as usize,
        flags as usize,
        fd as usize,
        offset as usize,
    );

    handle_result(arch_result)
}
