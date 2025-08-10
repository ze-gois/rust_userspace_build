pub fn copy(lhs: *mut u8, rhs: *const u8, n: usize) {
    unsafe { (0..n).for_each(|i| *lhs.add(i) = *rhs.add(i)) };
}
