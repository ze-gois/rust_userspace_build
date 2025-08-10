pub mod error {
    ::macros::define_error!("ELF", []);
}

::macros::define_error_nested! (
    "ELF",
    [
        [0; Elf;     self::error;     ERR_ELF;     "Local error";   "ERR_ELF"],
        [1; DType;   crate::dtype;    ERR_DTYPE;   "Datatype erro"; "ERR_DTYPE"],
        [2; Human;   human::result;   ERR_HUMAN;   "Human error";   "ERR_HUMAN"],
        [3; Syscall; syscall::result; ERR_SYSCALL; "Syscall error"; "ERR_SYSCALL"]
    ]
);
