pub mod ok {
    results::result!( Ok; "Human Ok"; usize; [
        [1; OK1; Ok; usize; "Ok"; "All good"],
        [1; OK2; OkMMap; usize; "Ok"; "All good"],
        [2; OK3; OkMUnMap; usize; "Ok"; "All good"],
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

pub fn handle_result(result: Result) -> crate::Result {
    match result {
        Ok(o) => core::result::Result::Ok(crate::Ok::from_no(o)),
        Err(e) => core::result::Result::Err(crate::Error::from_no(e)),
    }
}
