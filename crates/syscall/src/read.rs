use target::{Arch, traits::Callable};

hooking!(READ);

pub fn read(file_descriptor: isize, byte_buffer: *const u8, byte_length: usize) -> crate::Result {
    let arch_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_length as usize,
    );

    handle_result(arch_result)
}

pub mod ok {
    macros::r#struct!(OkSyscallMUnMap { value: usize });

    results::result!( Ok; "MUnMap Ok"; usize; [
        [0; OK; Ok; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Ok(no)
        }
    }
}

pub mod error {
    results::result!(Error; "MUnMap error"; usize; [
        [1; ERROR;          Error;            usize; "Error"; "Something wicked this way comes"],
        [4;   EINTR;        Interrupted;      usize;            "EINTR";     "System call was interrupted"],
        [5;   EIO;          IOError;          usize;              "EIO";       "Input/output error"],
        [9;   EBADF;        BadFileDescriptor;usize;            "EBADF";     "Bad file descriptor"],
        [14;  EFAULT;       InvalidBuffer;    usize;           "EFAULT";    "Invalid buffer pointer"],
        [22;  EINVAL;       InvalidCount;     usize;           "EINVAL";    "Invalid count"],
        [21;  EISDIR;       IsDirectory;      usize;           "EISDIR";    "Is a directory"],
        [13;  EACCES;       NotReadable;      usize;           "EACCES";    "File not open for reading"],
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

pub fn handle_result(result: target::Result) -> crate::Result {
    match result {
        Ok(o) => match o {
            target::Ok::Ok(no) => core::result::Result::Ok(crate::Ok::Read(Ok::Ok(no))),
        },
        Err(e) => match e {
            target::Error::Error(no) => core::result::Result::Err(crate::Error::Error(no)),
        },
    }
}
