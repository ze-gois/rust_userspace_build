#[unsafe(no_mangle)]
pub extern "C" fn flag_license(stack_pointer: crate::target::arch::PointerType) -> () {
    crate::file::print("LICENSE");
    unsafe { core::arch::asm!("mov {0}, rdi", in(reg) stack_pointer) };
}
