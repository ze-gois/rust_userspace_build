use crate::result::{ErrorType, Result, handle_result};

#[inline(always)]
pub fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> Result {
    let syscall_return: ErrorType;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            out("rcx") _,
            out("r11") _,
        );
    }
    handle_result(syscall_return)
}
