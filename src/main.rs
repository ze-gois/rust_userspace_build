#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: arch::PointerType) -> ! {
    xelf::info!("eXecuting Executable and Linkable Format\n\n");

    let stack = arch::memory::Stack::from_pointer(arch::Pointer(stack_pointer));
    stack.print();

    let file_path_pointer = stack.arguments.get(0);
    let file_path_pointer = file_path_pointer.unwrap();
    let file_path_pointer = file_path_pointer.pointer.0;

    let file_path_cstr = unsafe { core::ffi::CStr::from_ptr(file_path_pointer as *const i8) };
    let file_path = file_path_cstr.to_str().unwrap();

    let fd = common::file::open_path(file_path);

    let header_identifier_length = xelf::header::identifier::Index::NIdent.to();
    let header_identifier_pointer = arch::memory::alloc::<u8>(header_identifier_length as usize);

    let _ = syscall::read(
        fd,
        header_identifier_pointer,
        header_identifier_length as usize,
    );

    let _ = syscall::close(fd as i32);

    for b in 0..header_identifier_length {
        let byte = unsafe { *header_identifier_pointer.offset(b as isize) };
        xelf::info!("{:02X} ", byte);
    }
    xelf::info!("\n");

    ::common::file::print("LICENSE");

    let target_filapath = "/home/gois/branco\0";

    let tf_fd = match syscall::open(
        target_filapath.as_ptr(),
        syscall::open::Flag::RDWR.into(),
        0,
    ) {
        Ok(fd) => fd.0 as isize,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };

    // let tf_fd = fd;

    xelf::info!("\n{}\n", tf_fd);

    let tf_stat = syscall::mmap(
        core::ptr::null_mut(),
        core::mem::size_of::<syscall::fstat::Stat>(),
        syscall::mmap::Prot::Read | syscall::mmap::Prot::Write,
        syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private,
        -1,
        0,
    )
    .unwrap()
    .0 as *mut syscall::fstat::Stat;

    syscall::fstat(tf_fd as isize, tf_stat);

    let tf_stat = unsafe { *tf_stat };

    xelf::info!("{}: {:?}\n", tf_fd, tf_stat);

    let tf_content = syscall::mmap(
        core::ptr::null_mut(),
        tf_stat.st_size as usize,
        syscall::mmap::Prot::Read | syscall::mmap::Prot::Write,
        syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private,
        -1,
        0,
    )
    .unwrap()
    .0 as *mut u8;

    syscall::lseek(tf_fd as i32, 0, syscall::lseek::Flag::SET.to());
    syscall::read(tf_fd, tf_content, tf_stat.st_size as usize);
    syscall::write(1, tf_content, tf_stat.st_size as usize);

    xelf::info!("eXecuting Executable and Linkable Format\n");
    panic!("");
}
