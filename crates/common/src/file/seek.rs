pub fn seek(file_descriptor: isize, offset: i64) -> usize {
    match syscall::lseek(
        file_descriptor as i32,
        offset,
        syscall::lseek::Flag::SET as i32,
    ) {
        Ok(syscall::Ok::LSeek(syscall::lseek::Ok::Ok(pos))) => pos as usize,
        _ => 0,
    }
}
