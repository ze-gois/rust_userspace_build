// pub use crate::results::traits::Result as ResultTrait;

pub mod ok {
    results::result!(
        Ok;
        "x86_64 ok";
        usize;
        [
            [1; UNOTICED; UnoticedX86_64; usize; "Unoticed"; "We didn't evaluate it"],
            [2; UNOTICED_SYS3; UnoticedSys3X86_64; usize; "Unoticed"; "We didn't evaluate it"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::UnoticedX86_64(no)
        }
    }
}

pub mod error {
    results::result!(
        Error;
        "x86_64 error";
        usize;
        [
            [1; UNOTICED; UnoticedX86_64; usize; "Unoticed"; "We didn't evaluate it"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::UnoticedX86_64(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

// macros::r#enum!(pub Result, u8, [
//     [0, Ok, Ok],
//     [1, Err, Error]
// ]);

// pub fn handle_result(result: usize) -> crate::Result {
//     if (result as isize) < 0 {
//         Err(crate::Error::Syscall(crate::results::error::SyscallEntry{}))
//     } else {
//         Ok(Ok::from_no(result))
//     }
// }
