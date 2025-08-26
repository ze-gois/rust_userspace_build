#![no_std]
#![no_main]
#![allow(unused_variables)]

use userspace;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: arch::PointerType) -> ! {
    human::info!("Say whaaaaat?!");

    syscall::exit(32)
}
