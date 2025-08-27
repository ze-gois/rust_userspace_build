#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(unused_imports)]

use userspace;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: arch::PointerType) -> ! {
    human::info!("eXecuting Executable and Linkable Format\n\n");

    // let stack = memory::Stack::from_pointer(arch::Pointer(stack_pointer));
    // stack.print();

    // let file_path_pointer = stack.arguments.get(0);
    // let file_path_pointer = file_path_pointer.unwrap();
    // let file_path_pointer = file_path_pointer.pointer.0;

    // let file_path_cstr = unsafe { core::ffi::CStr::from_ptr(file_path_pointer as *const i8) };
    // let file_path = file_path_cstr.to_str().unwrap();

    // let fd = common::file::open_path(file_path);

    // let header_identifier_length = xelf::header::identifier::Index::NIdent(()).discriminant();
    // let header_identifier_pointer = memory::alloc::<u8>(header_identifier_length as usize);

    // let _ = syscall::read(
    //     fd,
    //     header_identifier_pointer,
    //     header_identifier_length as usize,
    // );

    // let _ = syscall::close(fd as i32);

    // for b in 0..header_identifier_length {
    //     let byte = unsafe { *header_identifier_pointer.offset(b as isize) };
    //     info!("{:02X} ", byte);
    // }
    // info!("\n");

    // ::common::file::print("LICENSE");

    // let target_filapath = "/home/gois/branco\0";

    // let tf_fd = match syscall::open(
    //     target_filapath.as_ptr(),
    //     syscall::open::Flag::RDWR.into(),
    //     0,
    // ) {
    //     Ok(syscall::Ok::Open(syscall::open::Ok::Ok(fd))) => fd as isize,
    //     Err(e) => panic!("Failed to open file: {:?}", e),
    //     _ => panic!("Failed to open file: {:?}", "NotOK"),
    // };

    // // let tf_fd = fd;

    // info!("\n{}\n", tf_fd);

    // let tf_stat = match syscall::mmap(
    //     core::ptr::null_mut(),
    //     core::mem::size_of::<syscall::fstat::Stat>(),
    //     (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
    //     (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private) as i32,
    //     -1,
    //     0,
    // ) {
    //     Ok(syscall::Ok::MMap(syscall::mmap::Ok::Ok(vma))) => vma as *mut syscall::fstat::Stat,
    //     _ => panic!("?"),
    // };
    // // .unwrap()
    // // .0 as *mut syscall::fstat::Stat;

    // syscall::fstat(tf_fd as isize, tf_stat);

    // let tf_stat = unsafe { *tf_stat };

    // info!("{}: {:?}\n", tf_fd, tf_stat);

    // let tf_content = match syscall::mmap(
    //     core::ptr::null_mut(),
    //     tf_stat.st_size as usize,
    //     (syscall::mmap::Prot::Read | syscall::mmap::Prot::Write) as i32,
    //     (syscall::mmap::Flag::Anonymous | syscall::mmap::Flag::Private) as i32,
    //     -1,
    //     0,
    // ) {
    //     Ok(syscall::Ok::MMap(syscall::mmap::Ok::Ok(vma))) => vma as *mut u8,
    //     _ => panic!("?"),
    // };

    // syscall::lseek(tf_fd as i32, 0, syscall::lseek::Flag::SET.to());
    // syscall::read(tf_fd, tf_content, tf_stat.st_size as usize);
    // syscall::write(1, tf_content, tf_stat.st_size as usize);

    human::info!("eXecuting Executable and Linkable Format\n");
    panic!("");
}
