use crate::arch::{Arch, traits::Callable};

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
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    results::result!(Error; "MUnMap error"; usize; [
        [1; ERROR; Default; usize; "Error"; "Something wicked this way comes"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: crate::Result) -> crate::Result {
    // Err(crate::Error::Default(1))
    match result {
        crate::Result::Ok(_ok) => match _ok {
            // crate::Ok::Arch(crate::arch::Ok::X86_64Syscall(crate::arch::syscall::Ok::X86_64Syscall1(crate::arch::syscall::syscall1::Ok::Default(no)))) => core::result::Result::Ok(crate::Ok::Os(crate::os::Ok::Syscall()
            _ => core::result::Result::Err(crate::Error::Info(2)),
        },
        crate::Result::Err(_) => core::result::Result::Err(crate::Error::Os(
            crate::os::Error::Syscall(crate::os::syscall::Error::LSeek(Error::Default(3))),
        )),
    }
}
