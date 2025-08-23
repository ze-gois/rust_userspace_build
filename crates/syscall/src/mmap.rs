use arch::{Arch, traits::Callable};

pub mod flags;

pub use flags::*;

hooking!(MMAP);

#[inline(always)]
#[rustfmt::skip]
pub fn mmap(addr: *mut u8, length: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> crate::Result {
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
    macros::r#struct!(OkSyscallMUnMap { value: usize });

    results::result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Ok; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Ok(no)
        }
    }
}

pub mod error {
    results::result!(Error; "MUnMap error"; usize; [
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

pub fn handle_result(result: arch::Result) -> crate::Result {
    match result {
        Ok(o) => match o {
            arch::Ok::Ok(no) => core::result::Result::Ok(crate::Ok::MMap(Ok::Ok(no))),
        },
        Err(e) => match e {
            arch::Error::Error(no) => core::result::Result::Err(crate::Error::Error(no)),
        },
    }
}
