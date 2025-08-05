use result::define_error;

define_error!(
    isize,
    "Human error",
    [
        [ZeEntry, 1, "Entry to ze", "ZE", ZE_ENTRY],
        [PeEntry, 2, "Entry to Pe", "ZE", PE_ENTRY],
    ]
);

pub fn handle_result(result: ErrorType) -> Result {
    if result < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(result)
    }
}
