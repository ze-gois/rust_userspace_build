pub mod macros;

use crate::elf_define_type;

pub mod endianness {
    macros::labeled_enum!(
        Endianness,
        isize,
        "endianness",
        [
            [None, NONE, 0, "No endianness provided", "no"],
            [LSB, LSB, 1, "No endianness provided", "no"],
            [MSB, MSB, 2, "No endianness provided", "no"],
            [Number, NUM, 3, "No endianness provided", "no"],
            [Undefined, UN, 4, "No endianness provided", "no"],
        ]
    );
}

pub use endianness::Endianness;

result::define_error!(
    "DType",
    [
        [InvalidData, 0, "No ", "NONE", NONE],
        [InvalidEndian, 1, "No endianness provided", "LSB", LSB],
        [InvalidType, 2, "No endianness provided", "MSB", MSB],
        [ShorterData, 3, "No endianness provided", "NUM", NUM],
    ]
);

pub trait ELFType {
    type Inner;
    const SIZE_BITS: usize;
    const SIZE_BYTES: usize;
}

elf_define_type!(pub UChar, u8); //Unsigned file offset
elf_define_type!(pub SXWord, i64); //Unsigned program address
elf_define_type!(pub Half, u16); //Unsigned medium integer
elf_define_type!(pub SWord, i32); //Unsigned integer
elf_define_type!(pub XWord, u64); //Signed integer
elf_define_type!(pub Word, u32); //Unsigned long integer
elf_define_type!(pub Off, u64); //Signed long integer
elf_define_type!(pub Addr, u64); //Unsigned small integer
