use target::{self, os::syscall};

pub fn alloc<T>(n: usize) -> *mut T {
    let t_size = core::mem::size_of::<T>();

    let aligned_size = n * t_size; //(t_size + ::memory::page::SIZE - 1) & !(::memory::page::SIZE - 1);

    match syscall::mmap(
        core::ptr::null_mut(),
        aligned_size,
        (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
        (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private) as i32,
        -1,
        0,
    ) {
        Ok(target::Ok::Os(target::os::Ok::Syscall(target::os::syscall::Ok::LSeek(
            target::os::syscall::lseek::Ok::Default(m),
        )))) => m as *mut T,
        _ => panic!("Failed to allocate memory"),
    }
}
