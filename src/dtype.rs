pub mod elf32;
pub mod elf64;
#[macro_use]
pub mod macros;

pub mod endianness {
    ::macros::labeled_enum!(
        Endianness,
        isize,
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

::macros::define_error!(
    "DType",
    [
        [0; InvalidData; NONE; "NONE";  "No " ],
        [1; InvalidEndian; LSB; "LSB";  "No endianness provided" ],
        [2; InvalidType; MSB; "MSB";  "No endianness provided" ],
        [3; ShorterData; NUM; "NUM";  "No endianness provided" ],
    ]
);

pub trait ELFType {
    type Inner;
    const SIZE_BITS: usize;
    const SIZE_BYTES: usize;
}
