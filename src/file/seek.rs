use target::os::syscall;

pub fn seek(file_descriptor: isize, offset: i64) -> usize {
    match syscall::lseek(
        file_descriptor as i32,
        offset,
        syscall::lseek::Flag::SET as i32,
    ) {
        Ok(target::Ok::Os(target::os::Ok::Syscall(target::os::syscall::Ok::LSeek(
            target::os::syscall::lseek::Ok::Default(m),
        )))) => m,
        _ => 0,
    }
}
