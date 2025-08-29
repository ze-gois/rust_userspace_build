use target;
use target::os::syscall;
pub fn terminate(head: &str) -> *const u8 {
    let tailed = match syscall::mmap(
        core::ptr::null_mut(),
        head.bytes().len() + 1,
        (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
        (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private) as i32,
        -1,
        0,
    ) {
        Ok(target::Ok::Os(target::os::Ok::Syscall(target::os::syscall::Ok::MMap(
            target::os::syscall::mmap::Ok::Default(m),
        )))) => m,
        _ => panic!("head"),
    };

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed as *mut u8, head.bytes().len());
        *(tailed as *mut u8).add(head.bytes().len()) = 0;
    };

    return tailed as *const u8;
}
