pub fn open_path(filepath: &str) -> isize {
    match filepath.chars().nth(0) {
        _ => {
            let dfd = syscall::open::AtFlag::FDCWD as isize;
            let flag = syscall::open::Flag::RDONLY as i32;
            let fd = match syscall::openat(dfd, filepath.as_ptr(), flag) {
                Ok(syscall::Ok::Open(syscall::open::Ok::OPENAT(no))) => no as isize,
                _ => -1,
            };
            fd
        }
    }
}
