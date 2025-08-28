#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(unused_imports)]

use userspace;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: target::arch::PointerType) -> ! {
    common::info!("Say whaaaaat?!");

    target::os::syscall::exit(32)
}
