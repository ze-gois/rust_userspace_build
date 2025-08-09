mod error {
    result::define_error!("Syscall", []);
}

result::define_error_nested!(
    "Syscall",
    [
        [Syscall; error; ERR_SYSCALL; 0; "Syscall auxiliary error"; "AUX"],
        [
            Close;
            crate::close;
            ERR_CLOSE;
            2;
            "Syscall close";
            "E_CLOSE"
        ],
        [
            LSeek;
            crate::lseek;
            ERR_LSEEK;
            3;
            "Syscall lseek";
            "E_LSEEK"
        ],
        [MMap; crate::mmap; ERR_MMAP; 4; "Syscall mmap"; "E_MMAP"],
        [
            MProtect;
            crate::mprotect;
            ERR_MPROTECT;
            5;
            "Syscall mprotect";
            "E_MPROTECT"
        ],
        [
            MUnmap;
            crate::munmap;
            ERR_MUNMAP;
            6;
            "Syscall munmap";
            "E_MUNMAP"
        ],
        [Open; crate::open; ERR_OPEN; 7; "Syscall open"; "E_OPEN"],
        [Read; crate::read; ERR_READ; 8; "Syscall read"; "E_READ"],
        [
            Write;
            crate::write;
            ERR_WRITE;
            9;
            "Syscall write";
            "E_WRITE"
        ],
        [
            FStat;
            crate::fstat;
            ERR_FSTAT;
            10;
            "Syscall fstat";
            "E_FSTAT"
        ],
    ]
);
