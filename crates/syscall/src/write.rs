use arch::{Arch, traits::Callable};

hooking!(WRITE);

use ::macros::define_error;

define_error!(
    "write",
    [
        [9;     BadFileDescriptor;  EBADF;   "EBADF";     "Bad file descriptor"],
        [14;    InvalidBuffer;      EFAULT;  "EFAULT";    "Invalid buffer pointer"],
        [27;    BufferTooLarge;     EFBIG;   "EFBIG";     "Buffer too large"],
        [4;     Interrupted;        EINTR;   "EINTR";     "System call was interrupted"],
        [5;     IOError;            EIO;     "EIO";       "Input/output error"],
        [28;    NoSpaceLeft;        ENOSPC;  "ENOSPC";    "No space left on device"],
        [32;    BrokenPipe;         EPIPE;   "EPIPE";     "Broken pipe"],
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
