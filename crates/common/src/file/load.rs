pub fn load(filepath: &str) -> Option<*const u8> {
    let filepath = crate::str::terminate(filepath);
    let license_mapping;
    'opening: loop {
        #[allow(unused_assignments)]
        let mut fd: usize = usize::MAX;
        'closing: loop {
            fd = match syscall::openat(
                syscall::open::flags::AtFlag::FDCWD as isize,
                filepath,
                syscall::open::flags::Flag::RDONLY as i32,
            ) {
                Ok(fd) => fd.1,
                Err(_) => {
                    break 'opening None;
                }
            };

            license_mapping = match syscall::mmap(
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

            let _ = syscall::read(fd as isize, license_mapping, 4096);
            break 'closing;
        }
        if fd != usize::MAX {
            // let _ = syscall::close(fd as i32);
            break 'opening Some(license_mapping);
        }
        break 'opening None;
    }
}
