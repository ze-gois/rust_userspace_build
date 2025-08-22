// use ::macros::result::ResultDefaultTrait;

// use ::macros::ZeroedMem;
pub mod traits;

mod error {
    results::result!(
        Error;
        "Human error";
        usize;
        [
            [1; ZE_ENTRY; ZeEntry; usize; "ZE"; "Entry to ze"],
            [2; PE_ENTRY; PeEntry; usize; "ZE"; "Entry to Pe"],
        ]
    );
}

mod ok {
    results::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [1; ZE_ENTRY; HumanOk; usize; "ZE"; "Entry to ze"],
        ]
    );
}

pub use error::Error;
pub use ok::Ok;

impl Error {
    pub fn from_no(no: usize) -> Self {
        Error::PeEntry(no)
    }
}

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: usize) -> Result {
    if (result as isize) < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(Ok::HumanOk(result))
    }
}
