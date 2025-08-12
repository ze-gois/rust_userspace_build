::macros::define_error!(
    "Human error",
    [
        [1; ZeEntry; ZE_ENTRY; "ZE"; "Entry to ze"],
        [2; PeEntry; PE_ENTRY; "ZE"; "Entry to Pe"],
    ]
);

pub fn handle_result(result: usize) -> Result {
    if (result as isize) < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(result)
    }
}
