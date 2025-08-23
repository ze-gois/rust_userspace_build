pub mod traits;

pub mod ok {

    crate::macros::r#struct!(pub OurStruct {
        value : usize,
        inform : u8,
    });

    results::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [1; ZE_ENTRY; HumanOk; usize; "ZE"; "Entry to ze"],
            [2; SYSCALL; SyscallOk; OurStruct; "ZE"; "Entry to ze"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::HumanOk(no)
        }
    }
}

pub mod error {
    crate::macros::r#struct!(pub SyscallEntry {
        value: usize,
        error: crate::arch::Error
    });

    results::result!(
        Error;
        "Human error";
        usize;
        [
            [1; ZE_ENTRY; ZeEntry; usize; "ZE"; "Entry to ze"],
            [2; SYSCALL; Syscall; SyscallEntry; "ZE"; "Entry to Pe"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Syscall(SyscallEntry {
                value: no,
                error: crate::arch::Error::Unoticed(no),
            })
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> Result {
    if (result as isize) < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(Ok::from_no(result))
    }
}
