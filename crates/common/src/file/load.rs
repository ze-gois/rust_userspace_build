pub fn load(filepath: &str) -> Option<(isize, syscall::fstat::Stat, *const u8)> {
    let filepath = crate::string::terminate(filepath);
    let license_mapping;
    'opening: loop {
        #[allow(unused_assignments)]
        let mut fd: isize = isize::MIN;
        let stat;
        'closing: loop {
            fd = match syscall::openat(
                syscall::open::flags::AtFlag::FDCWD as isize,
                filepath,
                syscall::open::flags::Flag::RDONLY as i32,
            ) {
                Ok(fd) => fd.1 as isize,
                Err(_) => {
                    break 'opening None;
                }
            };

            stat = crate::file::fstat(fd);

            ::human::info!("{:?}\n", stat);

            license_mapping = match syscall::mmap(
                core::ptr::null_mut(),
                stat.st_size as usize,
                syscall::mmap::Prot::Read | syscall::mmap::Prot::Write,
                syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private,
                -1,
                0,
            ) {
                Ok(m) => m.0 as *const u8,
                Err(_) => panic!("k"),
            };

            let _ = syscall::read(fd, license_mapping, stat.st_size as usize);
            break 'closing;
        }
        if fd >= 0 {
            // let _ = syscall::close(fd as i32);
            break 'opening Some((fd, stat, license_mapping));
        }
        break 'opening None;
    }
}
