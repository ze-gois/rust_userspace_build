pub unsafe fn length(s: *const u8) -> usize {
    let mut len = 0;

    while unsafe { *s.add(len) } != 0 {
        len += 1;
    }

    len
}
