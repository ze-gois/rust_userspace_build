use result::define_error;

define_error!("x86_64", []);

pub fn handle_result(result: usize) -> Result {
    human::info!("\n!!!! {} !!!!!\n", result);
    Ok(result)
}
