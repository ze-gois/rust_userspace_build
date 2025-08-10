pub fn fstat(fd: isize) -> syscall::fstat::Stat {
    let Ok((p, _)) = syscall::mmap(
        core::ptr::null_mut(),
        core::mem::size_of::<syscall::fstat::Stat>(),
        syscall::mmap::Prot::Read | syscall::mmap::Prot::Write,
        syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Shared,
        -1,
        0,
    )
    .map_err(|_e| panic!("ohones"));
    let p = p as *const syscall::fstat::Stat;
    let Ok(_) = syscall::fstat(fd as isize, p).map_err(|_e| panic!("natv"));

    unsafe { *p }
}
