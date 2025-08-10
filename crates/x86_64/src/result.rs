use ::macros::define_error;

define_error!("x86_64", []);

pub fn handle_result(result: usize) -> Result {
    Ok(result)
}
