use arch::{Arch, traits::Callable};

pub mod flags;
pub use flags::{AtFlag, Flag};

hooking!(OPEN);

pub fn open(file_pathname: *const u8, flags: i32, mode: i32) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        file_pathname as usize,
        flags as usize,
        mode as usize,
    );

    handle_result(syscall_result)
}

pub mod ok {
    macros::r#struct!(OkSyscallMUnMap { value: usize });

    results::result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Ok; usize; "Ok"; "All good"],
        [99; OPENAT;  OPENAT; usize; "OPENAT"; "WAITING"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Ok(no)
        }
    }
}

pub mod error {
    results::result!(Error; "MUnMap error"; usize; [
        [1;  ERROR;         Error;              usize;  "Error"; "Something wicked this way comes"],
        [2;  ENOENT;        FileNotFound;       usize;  "ENOENT";       "File not found"],
        [13; EACCES;        PermissionDenied;   usize;  "EACCES";       "Permission denied"],
        [22; EINVAL;        InvalidPath;        usize;  "EINVAL";       "Invalid path"],
        [20; ENOTDIR;       DirectoryNotFound;  usize;  "ENOTDIR";      "Directory not found"],
        [40; ELOOP;         TooManySymlinks;    usize;  "ELOOP";        "Too many levels of symbolic links"],
        [36; ENAMETOOLONG;  PathnameTooLong;    usize;  "ENAMETOOLONG"; "Pathname too long"],
        [17; EEXIST;        FileExists;         usize;  "EEXIST";       "File exists"],
        [24; EMFILE;        TooManyOpenFiles;   usize;  "EMFILE";       "Too many open files"],
        [28; ENOSPC;        NoSpace;            usize;  "ENOSPC";       "No space left on device"],
        [99; OPENAT;  OPENAT; usize; "OPENAT"; "WAITING"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Error(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

pub fn handle_result(result: arch::Result) -> crate::Result {
    match result {
        Ok(o) => match o {
            arch::Ok::Ok(no) => core::result::Result::Ok(crate::Ok::Open(Ok::Ok(no))),
        },
        Err(e) => match e {
            arch::Error::Error(no) => core::result::Result::Err(crate::Error::Error(no)),
        },
    }
}
