pub fn seek(file_descriptor: isize, offset: i64) -> usize {
    syscall::lseek(
        file_descriptor as i32,
        offset,
        syscall::lseek::Flag::SET as i32,
    )
    .unwrap()
    .0
}
