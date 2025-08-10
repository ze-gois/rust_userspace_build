#![no_std]
#![no_main]

use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n");

    let stack = unsafe { arch::memory::Stack::from_pointer(stack_pointer) };
    stack.print();

    ::common::file::print("LICENSE");
    xelf::info!("eXecuting Executable and Linkable Format\n");
    panic!("");
}
