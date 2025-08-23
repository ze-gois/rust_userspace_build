use crate::arch;

pub fn handle_result(syscall_return: usize) -> arch::Result {
    arch::Result::Ok(arch::Ok::UnoticedSys3X86_64(syscall_return))
}

#[inline(always)]
pub fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> arch::Result {
    let syscall_return: usize;

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            out("rcx") _,
            out("r11") _,
            lateout("rax") syscall_return,
        );
    }

    handle_result(syscall_return)
}
