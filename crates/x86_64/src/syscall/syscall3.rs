use crate::result::{Result, handle_result};

#[inline(always)]
pub fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> Result {
    let syscall_return: usize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            out("rcx") _,
            out("r11") _,
        );
    }
    human::info!("syscall3 from x86_64: {}", syscall_return);
    handle_result(syscall_return)
}
