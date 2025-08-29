#[unsafe(no_mangle)]
pub extern "C" fn flag_license() -> () {
    crate::file::print("LICENSE");
}
