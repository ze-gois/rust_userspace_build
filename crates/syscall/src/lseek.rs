use arch::{Arch, traits::Callable};

pub mod flags;
pub use flags::Flag;

hooking!(LSEEK);

#[inline(always)]
pub fn lseek(fd: i32, offset: i64, whence: i32) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, fd as usize, offset as usize, whence as usize);

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
            arch::Ok::Ok(no) => core::result::Result::Ok(crate::Ok::LSeek(Ok::Ok(no))),
        },
        Err(e) => match e {
            arch::Error::Error(no) => core::result::Result::Err(crate::Error::Error(no)),
        },
    }
}
