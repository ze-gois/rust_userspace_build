use super::Number;
use arch::{Arch, Callable};

pub mod flags;
pub use flags::{AtFlag, Flag};

static NUMBER: usize = Number::OpenAt as usize;

use result::define_error;

define_error!(
    "openat",
    [
        [FileNotFound, 2, "File not found", "ENOENT", ENOENT],
        [PermissionDenied, 13, "Permission denied", "EACCES", EACCES],
        [InvalidPath, 22, "Invalid path", "EINVAL", EINVAL],
        [
            DirectoryNotFound,
            20,
            "Directory not found",
            "ENOTDIR",
            ENOTDIR
        ],
        [
            TooManySymlinks,
            40,
            "Too many levels of symbolic links",
            "ELOOP",
            ELOOP
        ],
        [
            PathnameTooLong,
            36,
            "Pathname too long",
            "ENAMETOOLONG",
            ENAMETOOLONG
        ],
        [FileExists, 17, "File exists", "EEXIST", EEXIST],
        [
            TooManyOpenFiles,
            24,
            "Too many open files",
            "EMFILE",
            EMFILE
        ],
        [NoSpace, 28, "No space left on device", "ENOSPC", ENOSPC]
    ]
);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::Open(Error::TODO)),
        Ok(no) => match no {
            _ => {
                human::info!("273498374 +++{}n", no);
                Ok((no, no))
            }
        },
    }
}

pub fn openat(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
    );

    handle_result(syscall_result)
}

pub fn openat4(
    directory_file_descriptor: isize,
    file_pathname: *const u8,
    flags: i32,
    mode: i32,
) -> crate::Result {
    let syscall_result = Arch::syscall4(
        NUMBER,
        directory_file_descriptor as usize,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );
    handle_result(syscall_result)
}
