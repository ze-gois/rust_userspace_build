use arch::{Arch, traits::Callable};

const NUMBER: usize = super::numbers::EXIT;

pub fn exit(status_code: i32) -> ! {
    let status_code = status_code as usize;

    unsafe {
        let _ = Arch::syscall1(NUMBER, status_code);
        core::hint::unreachable_unchecked()
    }
}
