#![no_std]
pub use human::info;

pub mod result;
pub use result::{Error, Result};

#[macro_use]
pub mod macros;

pub use arch::traits::callable;

publishing!(
    [0; "Read"; READ; read; callable::Syscall3],
    [1; "Write"; WRITE; write; callable::Syscall3],
    [2; "Open"; OPEN; open; callable::Syscall3],
    [3; "Close"; CLOSE; close; callable::Syscall1],
    [5; "FStat"; FSTAT; fstat; callable::Syscall3],
    [8; "LSeek"; LSEEK; lseek; callable::Syscall3],
    [9; "MMap"; MMAP; mmap; callable::Syscall6],
    [10; "MProtect"; MPROTECT; mprotect; callable::Syscall3],
    [11; "MUnMap"; MUNMAP; munmap; callable::Syscall2],
    [60; "Exit"; EXIT; exit; callable::Syscall1],
    [257; "OpenAt"; OPENAT; openat; callable::Syscall3],
    [258; "OpenAt4"; OPENAT4; openat4; callable::Syscall4]
);
