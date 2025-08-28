pub mod ok {
    macros::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [0; LINUX_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
            [1; LINUX_SYSCALL_OK; Syscall; super::super::syscall::Ok; "ZE"; "Entry to ze"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    macros::result!(
        Error;
        "Human error";
        usize;
        [
            [0; LINUX_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
            [1; LINUX_SYSCALL_ERROR; Syscall; super::super::syscall::Error; "ZE"; "Entry to ze"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        Err(crate::Error::Os(Error::from_no(result)))
    } else {
        Ok(crate::Ok::Os(Ok::from_no(result)))
    }
}
