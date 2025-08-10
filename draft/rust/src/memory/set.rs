pub unsafe fn set(lhs: *mut u8, value: u8, n: usize) {
    unsafe { (0..n).for_each(|i| *lhs.add(i) = value) };
}
