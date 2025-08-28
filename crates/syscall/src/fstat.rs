use target::{Arch, traits::Callable};

pub mod stat;
pub use stat::Stat;

hooking!(FSTAT);

#[inline(always)]
pub fn fstat(fd: isize, stat: *const Stat) -> crate::Result {
    let arch_result = Arch::syscall2(NUMBER, fd as usize, stat as usize);
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

pub fn handle_result(result: target::Result) -> crate::Result {
    match result {
        Ok(o) => match o {
            target::Ok::Ok(no) => core::result::Result::Ok(crate::Ok::FStat(Ok::Ok(no))),
        },
        Err(e) => match e {
            target::Error::Error(no) => core::result::Result::Err(crate::Error::Error(no)),
        },
    }
}
