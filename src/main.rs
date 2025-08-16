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
    xelf::info!("eXecuting Executable and Linkable Format\n");
    panic!("");
}
