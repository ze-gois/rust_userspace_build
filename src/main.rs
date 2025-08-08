#![no_std]
#![no_main]

use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n");

    // Create a Stack instance from the provided pointer
    let stack = unsafe { arch::memory::Stack::from_pointer(stack_pointer) };
    xelf::info!("Stack from pointer:\n");
    stack.print();

    let x = xelf::dtype::Half::from(32);
    xelf::info!("{}", x);

    let xer = xelf::result::Error::DType(xelf::dtype::Error::InvalidType);

    panic!("Stack demonstration completed successfully!");
}
