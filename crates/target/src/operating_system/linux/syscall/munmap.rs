use crate::arch::{Arch, traits::Callable};

hooking!(MUNMAP);

#[inline(always)]
pub fn munmap(addr: *mut u8, length: usize) -> crate::Result {
    let arch_result = Arch::syscall2(NUMBER, addr as usize, length);
    handle_result(arch_result)
}

pub mod ok {
    macros::r#struct!(OkSyscallMUnMap { value: usize });

    macros::result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    macros::result!(Error; "MUnMap error"; usize; [
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
        crate::Result::Ok(crate::Ok::Arch(crate::arch::Ok::X86_64Syscall(
            crate::arch::syscall::Ok::X86_64Syscall2(crate::arch::syscall::syscall2::Ok::Default(
                m,
            )),
        ))) => core::result::Result::Ok(crate::Ok::Os(crate::os::Ok::Syscall(
            crate::os::syscall::Ok::MUnmap(crate::os::syscall::munmap::Ok::Default(m)),
        ))),
        _ => core::result::Result::Err(crate::Error::Os(crate::os::Error::Syscall(
            crate::os::syscall::Error::MUnmap(Error::Default(3)),
        ))),
    }
}
