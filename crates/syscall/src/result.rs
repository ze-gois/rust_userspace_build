pub mod traits {
    results::trait_place_result!();
}

// mod auxiliar {
//     ::macros::define_error!("auxiliar", []);
// }

// ::macros::define_error_nested!(
//     "Syscall",
//     [

//     ]
// );

pub mod ok {
    macros::r#struct!(MMapOk {});
    results::result!( Ok; "MMap Ok"; usize; [
        [0; OK;         Ok; usize; "Ok"; "All good"],
        [2; ERR_CLOSE;  Close;    crate::close::Ok;        "close";     "E_CLOSE" ],
        [3; ERR_LSEEK;  LSeek;    crate::lseek::Ok;        "lseek";     "E_LSEEK"],
        [4; ERR_MMAP;  MMap;     crate::mmap::Ok;          "mmap";      "E_MMAP"],
        [5; ERR_MPROTECT;  MProtect; crate::mprotect::Ok;  "mprotect";  "E_MPROTECT"],
        [6; ERR_MUNMAP;  MUnmap;   crate::munmap::Ok;      "munmap";    "E_MUNMAP"],
        [7; ERR_OPEN;  Open;     crate::open::Ok;          "open";      "E_OPEN"],
        [8; ERR_READ;  Read;     crate::read::Ok;          "read";      "E_READ"],
        [9; ERR_WRITE;  Write;    crate::write::Ok;        "write";     "E_WRITE"],
        [10; ERR_FSTAT; FStat;    crate::fstat::Ok;        "fstat";     "E_FSTAT"]
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Ok(no)
        }
    }
}

pub mod error {
    results::result!(Error; "MMap error"; usize; [
        [1; ERROR;      Error; usize; "Error"; "Something wicked this way comes"],
        [2; ERR_CLOSE;  Close;    crate::close::Error;        "close";     "E_CLOSE" ],
        [3; ERR_LSEEK;  LSeek;    crate::lseek::Error;        "lseek";     "E_LSEEK"],
        [4; ERR_MMAP;  MMap;     crate::mmap::Error;          "mmap";      "E_MMAP"],
        [5; ERR_MPROTECT;  MProtect; crate::mprotect::Error;  "mprotect";  "E_MPROTECT"],
        [6; ERR_MUNMAP;  MUnmap;   crate::munmap::Error;      "munmap";    "E_MUNMAP"],
        [7; ERR_OPEN;  Open;     crate::open::Error;          "open";      "E_OPEN"],
        [8; ERR_READ;  Read;     crate::read::Error;          "read";      "E_READ"],
        [9; ERR_WRITE;  Write;    crate::write::Error;        "write";     "E_WRITE"],
        [10; ERR_FSTAT; FStat;    crate::fstat::Error;        "fstat";     "E_FSTAT"]
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
