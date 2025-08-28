pub fn load(filepath: &str) -> Option<(isize, syscall::fstat::Stat, *const u8)> {
    let filepath = crate::types::string::terminate(filepath);
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
                Ok(syscall::Ok::Open(syscall::open::Ok::OPENAT(no))) => no as isize,
                _ => break 'opening None,
            };

            stat = crate::file::information::from_fd(fd);

            // crate::info!("{:?}\n", stat);

            license_mapping = match syscall::mmap(
                core::ptr::null_mut(),
                stat.st_size as usize,
                (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
                (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private) as i32,
                -1,
                0,
            ) {
                Ok(syscall::Ok::MMap(mmap_ok)) => match mmap_ok {
                    syscall::mmap::Ok::Default(no) => no as *const u8,
                },
                _ => {
                    crate::info!("Failed to mmap file");
                    panic!("k")
                }
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
