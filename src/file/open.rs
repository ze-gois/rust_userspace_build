use target::os::syscall;

pub fn open(filepath: &str) -> isize {
    match filepath.chars().nth(0) {
        _ => {
            let dfd = syscall::open::AtFlag::FDCWD as isize;
            let flag = syscall::open::Flag::RDONLY as i32;
            let fd = match syscall::openat(dfd, filepath.as_ptr(), flag) {
                Ok(target::Ok::Os(target::os::Ok::Syscall(target::os::syscall::Ok::Open(
                    target::os::syscall::open::Ok::Default(m),
                )))) => m as isize,
                _ => -1,
            };
            fd
        }
    }
}
