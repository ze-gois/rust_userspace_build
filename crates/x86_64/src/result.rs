use result::define_error;

define_error!("X86_64", []);

pub fn handle_result(result: usize) -> Result {
    if (result as isize) < 0 {
        Err(Error::from_no(result))
    } else {
        Ok(result)
    }
}
