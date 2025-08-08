#![no_std]
#![no_main]

use arch::info;
use xelf;

#[unsafe(no_mangle)]
pub extern "C" fn entry(stack_pointer: *mut u64) -> ! {
    // xelf::info!("eXecuting Executable and Linkable Format\n");

    // Create a Stack instance from the provided pointer
    // let mut stack = unsafe { arch::memory::Stack::from_pointer(stack_pointer) };
    // xelf::info!("Stack from pointer:\n");
    // stack.print();

    // Access specific stack elements
    // unsafe {
    //     if let Some(arg0) = stack.get_arg(0) {
    //         xelf::info!("Program name: {}\n", arg0);
    //     }

    //     // Check for specific arguments
    //     for i in 0..stack.argc {
    //         if let Some(arg) = stack.get_arg(i) {
    //             if arg == "/" {
    //                 xelf::info!("Found '/' at argument position {}\n", i);
    //             }
    //         }
    //     }

    //     // Get environment variables
    //     if let Some(path) = stack.get_env_by_name("PATH") {
    //         xelf::info!("PATH environment variable: {}\n", path);
    //     }

    //     // Get and modify auxiliary vector entries
    //     if let Some(entry_point) = stack.get_auxv_by_type(9) {
    //         // AT_ENTRY
    //         xelf::info!("Original entry point: {:#x}\n", entry_point);

    //         // Modify an auxiliary vector entry (example only, not actually changing it)
    //         if stack.set_auxv_by_type(9, 0x11502) {
    //             xelf::info!("Entry point updated (demonstrative only)\n");
    //         }
    //     }
    // }

    let mut fd;
    'license: loop {
        'opening: loop {
            info!("opening\n");
            let license_fd = syscall::openat(
                syscall::open::flags::AtFlag::FDCWD.into(),
                "build.rs".as_ptr(),
                syscall::open::flags::Flag::RDONLY.into(),
            );
            info!("opening the egg");
            fd = match license_fd {
                Ok(fd) => {
                    info!("\nwedo\n");
                    fd.1
                }
                Err(e) => {
                    use result::ErrorNestedTrait;
                    info!("{:?}", e.to_no());
                    break 'opening;
                }
            };
            info!("---------------");

            let x = &[0_u8; 100];
            let r = syscall::read(fd as isize, x.as_ptr(), 100);
            let xx = x.as_ptr();
            info!("\n\n>>>>>\n");
            let w = syscall::write(1, xx, 100);
            info!("\n>>>>>\n\n");
            let _ = syscall::close(fd as i32);
            break 'opening;
        }
        break 'license;
    }

    xelf::info!("\nDemonstration complete\n");

    // stack.print();
    panic!("Stack demonstration completed successfully!");
}
