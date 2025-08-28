use target::{Arch, traits::Callable};

hooking!(WRITE);

pub fn write(file_descriptor: isize, byte_buffer: *const u8, byte_count: usize) -> crate::Result {
    let syscall_result = Arch::syscall3(
        NUMBER,
        file_descriptor as usize,
        byte_buffer as usize,
        byte_count as usize,
    );

    handle_result(syscall_result)
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
        [1;  ERROR;   Error;             usize; "Error"; "Something wicked this way comes"],
        [9;  EBADF;   BadFileDescriptor; usize;   "EBADF";     "Bad file descriptor"],
        [14; EFAULT;  InvalidBuffer;     usize;  "EFAULT";    "Invalid buffer pointer"],
        [27; EFBIG;   BufferTooLarge;    usize;   "EFBIG";     "Buffer too large"],
        [4;  EINTR;   Interrupted;       usize;   "EINTR";     "System call was interrupted"],
        [5;  EIO;     IOError;           usize;     "EIO";       "Input/output error"],
        [28; ENOSPC;  NoSpaceLeft;       usize;  "ENOSPC";    "No space left on device"],
        [32; EPIPE;   BrokenPipe;        usize;   "EPIPE";     "Broken pipe"],
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
            target::Ok::Ok(no) => core::result::Result::Ok(crate::Ok::Write(Ok::Ok(no))),
        },
        Err(e) => match e {
            target::Error::Error(no) => core::result::Result::Err(crate::Error::Error(no)),
        },
    }
}
