#![no_std]
#![no_main]

use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n");

    let x = ::common::file::load("Cargo.toml");

    let _ = syscall::write(1, x.unwrap(), 100);

    panic!("Stack demonstration completed successfully!");
}
