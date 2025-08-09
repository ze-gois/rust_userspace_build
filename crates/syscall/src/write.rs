use arch::{Arch, traits::Callable};

hooking!(WRITE);

use result::define_error;

define_error!(
    "write",
    [
        [BadFileDescriptor, 9, "Bad file descriptor", "EBADF", EBADF],
        [
            InvalidBuffer,
            14,
            "Invalid buffer pointer",
            "EFAULT",
            EFAULT
        ],
        [BufferTooLarge, 27, "Buffer too large", "EFBIG", EFBIG],
        [
            Interrupted,
            4,
            "System call was interrupted",
            "EINTR",
            EINTR
        ],
        [IOError, 5, "Input/output error", "EIO", EIO],
        [NoSpaceLeft, 28, "No space left on device", "ENOSPC", ENOSPC],
        [BrokenPipe, 32, "Broken pipe", "EPIPE", EPIPE]
    ]
);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::Write(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

pub fn write(file_descriptor: isize, byte_buffer: *const u8, byte_count: usize) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(syscall_result)
}
