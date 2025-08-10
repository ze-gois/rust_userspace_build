pub fn print() {
    pub const FILEPATH: &str = "LICENSE";

    let fd = match syscall::openat(
        syscall::open::flags::AtFlag::FDCWD as isize,
        FILEPATH.as_ptr(),
        syscall::open::flags::Flag::RDONLY as i32,
    ) {
        Ok(fd) => fd.1,
        Err(_) => panic!("'l'"),
    };

    let file_pointer = match syscall::mmap(
        core::ptr::null_mut(),
        4096,
        syscall::mmap::Prot::Read | syscall::mmap::Prot::Write,
        syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private,
        -1,
        0,
    ) {
        Ok(m) => m.0 as *const u8,
        Err(_) => panic!("k"),
    };

    let _ = syscall::read(fd as isize, file_pointer, 4096);
    let _ = syscall::write(1, file_pointer, 4096);
    let _ = syscall::close(fd as i32);
}
