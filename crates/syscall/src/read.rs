use arch::{Arch, Callable};

hooking!(READ);

use result::define_error;

define_error!(
    "Read",
    [
        [BadFileDescriptor, 9, "Bad file descriptor", "EBADF", EBADF],
        [
            InvalidBuffer,
            14,
            "Invalid buffer pointer",
            "EFAULT",
            EFAULT
        ],
        [InvalidCount, 22, "Invalid count", "EINVAL", EINVAL],
        [
            Interrupted,
            4,
            "System call was interrupted",
            "EINTR",
            EINTR
        ],
        [IOError, 5, "Input/output error", "EIO", EIO],
        [IsDirectory, 21, "Is a directory", "EISDIR", EISDIR],
        [
            NotReadable,
            13,
            "File not open for reading",
            "EACCES",
            EACCES
        ],
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
