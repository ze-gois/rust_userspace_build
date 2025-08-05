use crate::result::{ErrorType, Result, handle_result};

#[inline(always)]
pub fn syscall0(n: usize) -> Result {
    let syscall_return: ErrorType;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            out("rcx") _,
            out("r11") _,
        );
    }
    handle_result(syscall_return)
}
