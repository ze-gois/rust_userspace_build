use result::{define_error, define_error_nested};

use crate::dtype::Error as DTypeError;

// use human::result::Error as HumanError;
// use syscall::result::Error as SyscallError;

mod result {
    define_error!();
}

define_error_nested!(
    "ELF",
    [
        [Elf, self::result, ELF, 0, "Local error", ELF_ERR],
        [Human, human::result, HUMAN, 1, "Human error", HUMAN_ERR],
        [
            Syscall,
            syscall::result,
            SYSCALL,
            2,
            "Syscall error",
            SYSCALL_ERR
        ],
    ]
);

// macros::labeled_typed_enum!(
//     Error,
//     isize,
//     "ELF Errors",
//     [
//         [
//             Syscall,
//             SyscallError,
//             ERR_SYSCALL,
//             0,
//             "Error ocurring from a syscall",
//             "ErrSys"
//         ],
//         [
//             DType,
//             DTypeError,
//             ERR_DTYPE,
//             1,
//             "Error ocurring from a datatype",
//             "DType"
//         ],
//         [
//             Human,
//             HumanError,
//             ERR_HUMAN,
//             2,
//             "Error ocurring from a human interface",
//             "Hum"
//         ],
//     ]
// );

// #[repr(isize)]
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum Error {
//     Syscall(SyscallError),
//     Human(HumanError),
//     DType(DTypeError),
//     TODO,
// }

// impl result::ErrorTrait for Error {
//     fn from_no(errno: isize) -> Error {
//         match -errno {
//             _ => Error::TODO,
//         }
//     }

//     fn describe(&self) -> &str {
//         match self {
//             _ => "TODO",
//         }
//     }

//     fn advert(&self) -> Option<isize> {
//         match self {
//             _ => None,
//         }
//     }
// }

// impl Into<isize> for Error {
//     fn into(self) -> isize {
//         match self {
//             Error::Human(_e) => -4,
//             Error::Syscall(_e) => -2,
//             Error::DType(_e) => -3,
//             Error::TODO => -1,
//         }
//     }
// }

pub type Result<T> = core::result::Result<T, Error>;
