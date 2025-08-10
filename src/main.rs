#![no_std]
#![no_main]

use xelf::{self};

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n");

    let (fd, fstat, content) = ::common::file::load("LICENSE").unwrap();
    xelf::info!("{:?}", fstat);
    let _ = syscall::write(1, content, fstat.st_size as usize);

    panic!("Stack demonstration completed successfully!");
}
