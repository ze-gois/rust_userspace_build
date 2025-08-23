pub fn fstat(fd: isize) -> syscall::fstat::Stat {
    let p = match syscall::mmap(
        core::ptr::null_mut(),
        core::mem::size_of::<syscall::fstat::Stat>(),
        (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
        (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Shared) as i32,
        -1,
        0,
    ) {
        Ok(syscall::Ok::MMap(syscall::mmap::Ok::Ok(no))) => no as *const u8,
        _ => panic!("ohones"),
    };

    let p = p as *const syscall::fstat::Stat;
    let Ok(_) = syscall::fstat(fd as isize, p).map_err(|_e| panic!("natv"));

    unsafe { *p }
}
