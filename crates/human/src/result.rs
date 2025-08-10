use ::macros::define_error;

define_error!(
    "Human error",
    [
        [ZeEntry, 1, "Entry to ze", "ZE", ZE_ENTRY],
        [PeEntry, 2, "Entry to Pe", "ZE", PE_ENTRY],
    ]
);

pub fn handle_result(result: usize) -> Result {
    if (result as isize) < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(result)
    }
}
