use arch::{Arch, traits::Callable};

hooking!(READ);

use ::macros::define_error;

define_error!(
    "Read",
    [
        [4;   Interrupted;       EINTR;    "EINTR";     "System call was interrupted"],
        [5;   IOError;           EIO;      "EIO";       "Input/output error"],
        [9;   BadFileDescriptor; EBADF;    "EBADF";     "Bad file descriptor"],
        [14;  InvalidBuffer;     EFAULT;   "EFAULT";    "Invalid buffer pointer"],
        [22;  InvalidCount;      EINVAL;   "EINVAL";    "Invalid count"],
        [21;  IsDirectory;       EISDIR;   "EISDIR";    "Is a directory"],
        [13;  NotReadable;       EACCES;   "EACCES";    "File not open for reading"],
    ]
);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::Read(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

pub fn read(file_descriptor: isize, byte_buffer: *const u8, byte_length: usize) -> crate::Result {
    let arch_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_length as usize,
    );

    handle_result(arch_result)
}
