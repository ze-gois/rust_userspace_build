use super::map;

pub fn alloc<T>(n: usize) -> *mut T {
    let t_size = core::mem::size_of::<T>();

    let aligned_size = n * t_size; //(t_size + ::memory::page::SIZE - 1) & !(::memory::page::SIZE - 1);

    let Ok((nt_pointer, _)) = map::mmap(
        core::ptr::null_mut(),
        aligned_size,
        (map::Prot::Read | map::Prot::Write) as i32,
        (map::Flag::Anonymous | map::Flag::Private) as i32,
        -1,
        0,
    )
    .map_err(|_| panic!("er"));

    nt_pointer as *mut T
}
