use crate::close::Error as CloseError;
use crate::lseek::Error as LSeekError;
use crate::mmap::Error as MMapError;
use crate::mprotect::Error as MProtectError;
use crate::munmap::Error as MUnMapError;
use crate::open::Error as OpenError;
use crate::read::Error as ReadError;
use crate::write::Error as WriteError;

use result::define_error_nested;

define_error_nested!(
    "Syscall",
    [
        [Proc, self, ERR_PROC, 0, "Syscall process", "E_PROC"],
        [
            Close,
            crate::close,
            ERR_CLOSE,
            1,
            "Syscall close",
            "E_CLOSE"
        ],
        [
            Lseek,
            crate::lseek,
            ERR_LSEEK,
            1,
            "Syscall lseek",
            "E_LSEEK"
        ],
        [Mmap, crate::mmap, ERR_MMAP, 1, "Syscall mmap", "E_MMAP"],
        [
            Mprotect,
            crate::mprotect,
            ERR_MPROTECT,
            1,
            "Syscall mprotect",
            "E_MPROTECT"
        ],
        [
            Munmap,
            crate::munmap,
            ERR_MUNMAP,
            1,
            "Syscall munmap",
            "E_MUNMAP"
        ],
        [Open, crate::open, ERR_OPEN, 1, "Syscall open", "E_OPEN"],
        [Read, crate::read, ERR_READ, 1, "Syscall read", "E_READ"],
        [
            Write,
            crate::write,
            ERR_WRITE,
            1,
            "Syscall write",
            "E_WRITE"
        ],
    ]
);

// macros::labeled_typed_enum!(
//     Error,
//     isize,
//     "Syscall Error",
//     [
//         [Open, OpenError, ERR_OPEN, 0, "Some Open error", "e_open"],
//         [Read, ReadError, ERR_READ, 1, "Some Read error", "e_read"],
//         [
//             Write,
//             WriteError,
//             ERR_WRITE,
//             2,
//             "Some Write error",
//             "e_write"
//         ],
//         [
//             LSeek,
//             LSeekError,
//             ERR_LSEEK,
//             3,
//             "Some LSeek error",
//             "e_lseek"
//         ],
//         [MMap, MMapError, ERR_MMAP, 4, "Some MMap error", "e_mmap"],
//         [
//             Close,
//             CloseError,
//             ERR_CLOSE,
//             5,
//             "Some Close error",
//             "e_close"
//         ],
//         [
//             MProtect,
//             MProtectError,
//             ERR_MPROTECT,
//             6,
//             "Some MProtect error",
//             "e_mprotect"
//         ],
//         [
//             MUnMap,
//             MUnMapError,
//             ERR_MUNMAP,
//             7,
//             "Some MUnMap error",
//             "e_munmap"
//         ],
//     ]
// );
//     ,
//     ,
//     ,
//     ,
//     ,
//     ,
//     ,
//     ,

// #[repr(isize)]
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum Error {
//     Open(OpenError),
//     Read(ReadError),
//     Write(WriteError),
//     LSeek(LSeekError),
//     MMap(MMapError),
//     Close(CloseError),
//     MProtect(MProtectError),
//     MUnMap(MUnMapError),
//     TODO,
// }

// impl ErrorTrait for Error {
//     fn from_no(errno: isize) -> Self {
//         match -errno {
//             _ => Self::TODO,
//         }
//     }

//     fn describe(&self) -> &str {
//         match self {
//             Error::Open(err) => err.describe(),
//             Error::Read(err) => err.describe(),
//             Error::Write(err) => err.describe(),
//             Error::LSeek(err) => err.describe(),
//             Error::MMap(err) => err.describe(),
//             Error::Close(err) => err.describe(),
//             Error::MProtect(err) => err.describe(),
//             Error::MUnMap(err) => err.describe(),
//             Error::TODO => "TODO",
//         }
//     }

//     fn advert(&self) -> Option<isize> {
//         match self {
//             Error::Open(err) => err.advert(),
//             Error::Read(err) => err.advert(),
//             Error::Write(err) => err.advert(),
//             Error::LSeek(err) => err.advert(),
//             Error::MMap(err) => err.advert(),
//             Error::Close(err) => err.advert(),
//             Error::MProtect(err) => err.advert(),
//             Error::MUnMap(err) => err.advert(),
//             Error::TODO => None,
//         }
//     }
// }

// impl Into<isize> for Error {
//     fn into(self) -> isize {
//         match self {
//             _ => unsafe { *(&self as *const Self as *const isize) },
//         }
//     }
// }

// pub type Result<T> = core::result::Result<T, Error>;
