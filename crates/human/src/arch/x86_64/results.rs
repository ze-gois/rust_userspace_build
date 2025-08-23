// pub use crate::results::traits::Result as ResultTrait;

mod ok {
    results::result!(
        Ok;
        "Syscall ok";
        usize;
        [
            [1; UNOTICED; Unoticed; usize; "Unoticed"; "We didn't evaluate it"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Unoticed(no)
        }
    }
}

mod error {
    results::result!(
        Error;
        "Syscall error";
        usize;
        [
            [1; UNOTICED; Unoticed; usize; "Unoticed"; "We didn't evaluate it"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Unoticed(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        Err(crate::Error::)
    } else {
        Ok(Ok::from_no(result))
    }
}
