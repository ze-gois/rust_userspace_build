// use macros::result::ErrorTrait;

pub mod error {
    ::macros::define_error!("ELF", []);
}

::macros::define_error_nested! (
    "ELF",
    [
        [0; Elf;     self::error;     ERR_ELF;     "Local error";   "ERR_ELF"],
        // [1; DType;   crate::dtype;    ERR_DTYPE;   "Datatype erro"; "ERR_DTYPE"],
        [2; Human;   human::result;   ERR_HUMAN;   "Human error";   "ERR_HUMAN"],
        [3; Syscall; syscall::result; ERR_SYSCALL; "Syscall error"; "ERR_SYSCALL"]
    ]
);

// pub mod macro_types {
//     #[allow(non_camel_case_types)]
//     pub type Mi8 = *const i8;
//     pub type elf_error = super::error::Error;
//     pub type dtype_error = crate::dtype::Error;
// }

// mod ok {
//     ::macros::enum_typed!(Ok; usize; "AT_TYPE"; crate::result::macro_types; [
//         [0;    Null;            usize;  |p: Franco| { *p as usize };     AT_NULL;          "Null";          "End of vector"],
//         [1;    Ignore;          usize;  |p: Franco| { *p as usize };     AT_IGNORE;        "Ignore";        "Entry should be ignored"],
//     ]);
// }

// ::macros::error_typed!("ELF Type"; crate::result::macro_types; [
//    [0; Elf;     elf_error; p; { elf_error::from_ptr(p) };  { p.to_no() as Franco };                ERR_ELF; "ERR_ELF";   "ERR_ELF"],
//    [1; DType; dtype_error; p; { dtype_error::from_ptr(p)}; { p as *const dtype_error as Franco}; ERR_DTYPE; "ERR_DTYPE"; "ERR_DTYPE"],
// ]);

// pub type Result = core::result::Result<ok::Ok, Error>;
