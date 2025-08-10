mod auxiliar {
    ::macros::define_error!("auxiliar", []);
}

::macros::define_error_nested!(
    "Syscall",
    [
        [999; Auxiliar; self::auxiliar;  ERR_AUXILIAR; "auxiliary"; "E_AUX"],
        [2;   Close;    crate::close;    ERR_CLOSE;    "close";     "E_CLOSE" ],
        [3;   LSeek;    crate::lseek;    ERR_LSEEK;    "lseek";     "E_LSEEK"],
        [4;   MMap;     crate::mmap;     ERR_MMAP;     "mmap";      "E_MMAP"],
        [5;   MProtect; crate::mprotect; ERR_MPROTECT; "mprotect";  "E_MPROTECT"],
        [6;   MUnmap;   crate::munmap;   ERR_MUNMAP;   "munmap";    "E_MUNMAP"],
        [7;   Open;     crate::open;     ERR_OPEN;     "open";      "E_OPEN"],
        [8;   Read;     crate::read;     ERR_READ;     "read";      "E_READ"],
        [9;   Write;    crate::write;    ERR_WRITE;    "write";     "E_WRITE"],
        [10;  FStat;    crate::fstat;    ERR_FSTAT;    "fstat";     "E_FSTAT"]
    ]
);
