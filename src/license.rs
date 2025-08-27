#[unsafe(no_mangle)]
pub extern "C" fn flag_license() -> () {
    common::file::print("LICENSE");
}
