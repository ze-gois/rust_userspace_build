pub mod class_64;
pub use class_64::UChar;

pub mod traits;

#[macro_use]
pub mod macros;

pub mod endianness {
    ::macros::enum_labeled!(
        Endianness,
        u8,
        "endianness",
        [
            [0; None; NONE; "no"; "No endianness provided"],
            [1; LSB; LSB; "no"; "No endianness provided"],
            [2; MSB; MSB; "no"; "No endianness provided"],
            [3; Number; NUM; "no"; "No endianness provided"],
            [4; Undefined; UN; "no"; "No endianness provided"]
        ]
    );
}

pub use endianness::Endianness;

// ::macros::define_error!(
//     "DType",
//     [
//         [0; InvalidData; NONE; "NONE";  "No " ],
//         [1; InvalidEndian; LSB; "LSB";  "No endianness provided" ],
//         [2; InvalidType; MSB; "MSB";  "No endianness provided" ],
//         [3; ShorterData; NUM; "NUM";  "No endianness provided" ],
//     ]
// );

pub trait ELFType {
    type Inner;
    const SIZE_BITS: usize;
    const SIZE_BYTES: usize;
}

pub mod error_types {
    #[allow(non_camel_case_types)]
    pub type Mi8 = *const i8;
    pub type Franco = *const u8;
}

// crate::dtype::class_64

::macros::error_typed!("DType Error"; crate::dtype::error_types; [
    [0; Null;   u8;  p; { *p };                 {(p as *const u8) as Franco}; ERR_NULL;      "UChar_64";  "UChar_64"],
    [1; UChar;  u8;  p; { *(p as *const u8) };  {(p as *const u8) as Franco}; ERR_UCHAR_64;  "UChar_64";  "UChar_64"],
    [2; SXWord; i64; p; { *(p as *const i64) }; {(p as *const i64) as Franco}; ERR_SXWORD_64; "SXWord_64"; "SXWord_64"],
    [3; Half;   u16; p; { *(p as *const u16) }; {(p as *const u16) as Franco}; ERR_HALF_64;   "Half_64";   "Half_64"],
    [4; SWord;  i32; p; { *(p as *const i32) }; {(p as *const i32) as Franco}; ERR_SWORD_64;  "SWord_64";  "SWord_64"],
    [5; XWord;  u64; p; { *(p as *const u64) }; {(p as *const u64) as Franco}; ERR_XWORD_64;  "XWord_64";  "XWord_64"],
    [6; Word;   u32; p; { *(p as *const u32) }; {(p as *const u32) as Franco}; ERR_WORD_64;   "Word_64";   "Word_64"],
    [7; Off;    u64; p; { *(p as *const u64) }; {(p as *const u64) as Franco}; ERR_OFF_64;    "Off_64";    "Off_64"],
    [8; Addr;   u64; p; { *(p as *const u64) }; {(p as *const u64) as Franco}; ERR_ADDR_64;   "Addr_64";   "Addr_64"]
]);
