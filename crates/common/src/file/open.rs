pub fn open_path(filepath: &str) -> isize {
    match filepath.chars().nth(0) {
        _ => {
            let dfd = syscall::open::AtFlag::FDCWD as isize;
            let flag = syscall::open::Flag::RDONLY as i32;
            syscall::openat(dfd, filepath.as_ptr(), flag).unwrap().0 as isize
        }
    }
}
