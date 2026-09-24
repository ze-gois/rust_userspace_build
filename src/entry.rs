#![no_std]
#![no_main]

const LS_USR_BIN: &[u8] = b"/usr/bin/ls\0";
const LS_BIN: &[u8] = b"/bin/ls\0";

#[unsafe(no_mangle)]
pub extern "C" fn entry(
    stack_pointer: userspace_build::target::architecture::StackPointer,
) -> ! {
    let words = stack_pointer.cast::<usize>();
    let argument_count = unsafe { *words };

    // Linux initial stack:
    // argc, argv[argc], NULL, envp[], NULL, auxv...
    //
    // Reuse the current environment vector directly. execve(2) consumes the
    // pointed-to strings before replacing this process image.
    let environment = unsafe {
        words
            .add(argument_count.saturating_add(2))
            .cast::<*const u8>()
    };

    if execute(LS_USR_BIN, environment).is_err() {
        userspace_build::info!(
            "execve({:?}) failed; trying /bin/ls\n",
            "/usr/bin/ls",
        );

        if execute(LS_BIN, environment).is_err() {
            userspace_build::info!(
                "failed to execute ls through Linux execve\n"
            );
            userspace_build::target::operating_system::syscall::exit(127)
        }
    }

    // A successful execve never returns.
    userspace_build::target::operating_system::syscall::exit(127)
}

fn execute(
    path: &'static [u8],
    environment: *const *const u8,
) -> userspace_build::Result {
    let arguments = [
        path.as_ptr(),
        core::ptr::null(),
    ];

    userspace_build::target::operating_system::syscall::execve(
        path.as_ptr(),
        arguments.as_ptr(),
        environment,
    )
}
