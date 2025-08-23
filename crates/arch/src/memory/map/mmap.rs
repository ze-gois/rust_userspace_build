use crate::{Arch, Callable};

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

pub mod ok {
    results::result!( Ok; "MMap Ok"; usize; [
        [0; OK; OkMMap; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::OkMMap(no)
        }
    }
}

pub mod error {
    results::result!(Error; "MMap error"; usize; [
        [1; ERROR; Error; usize; "Error"; "Something wicked this way comes"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Error(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: crate::Result) -> super::Result {
    match result {
        Ok(o) => core::result::Result::Ok(super::Ok::OkMMap(32)),
        _ => core::result::Result::Err(super::Error::Error(12)),
    }
}
