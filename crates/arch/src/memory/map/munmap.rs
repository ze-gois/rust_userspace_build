use crate::{Arch, Callable};

#[inline(always)]
pub fn munmap(addr: *mut u8, length: usize) -> super::Result {
    let arch_result = Arch::syscall2(13, addr as usize, length);
    handle_result(arch_result)
}

pub mod ok {
    results::result!( Ok; "Human Ok"; usize; [
        [0; OK; Ok; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Ok(no)
        }
    }
}

pub mod error {
    results::result!(Error; "Human error"; usize; [
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
        Ok(o) => core::result::Result::Ok(super::Ok::OkMUnMap(32)),
        Err(o) => core::result::Result::Err(super::Error::Error(32)),
    }
}
