use arch::{Arch, traits::Callable};

pub mod flags;
pub use flags::{AtFlag, Flag};

hooking!(OPEN);

use ::macros::define_error;

#[rustfmt::skip]
define_error!(
    "open",
    [
        [2; FileNotFound;       ENOENT;         "ENOENT";       "File not found"],
        [13; PermissionDenied;  EACCES;         "EACCES";       "Permission denied"],
        [22; InvalidPath;       EINVAL;         "EINVAL";       "Invalid path"],
        [20; DirectoryNotFound; ENOTDIR;        "ENOTDIR";      "Directory not found"],
        [40; TooManySymlinks;   ELOOP;          "ELOOP";        "Too many levels of symbolic links"],
        [36; PathnameTooLong;   ENAMETOOLONG;   "ENAMETOOLONG"; "Pathname too long"],
        [17; FileExists;        EEXIST;         "EEXIST";       "File exists"],
        [24; TooManyOpenFiles;  EMFILE;         "EMFILE";       "Too many open files"],
        [28; NoSpace;           ENOSPC;         "ENOSPC";       "No space left on device"],
    ]
);

pub fn handle_result(arch_result: arch::Result) -> crate::Result {
    match arch_result {
        Err(arch::Error::TODO) => Err(crate::Error::Open(Error::TODO)),
        Ok(no) => match no {
            _ => Ok((no, no)),
        },
    }
}

pub fn open(file_pathname: *const u8, flags: i32, mode: i32) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );

    handle_result(syscall_result)
}
