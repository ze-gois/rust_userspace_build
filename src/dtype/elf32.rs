use super::ELFType;
use super::Endianness;

crate::elf_define_type!(pub UChar, u8); //Unsigned file offset
crate::elf_define_type!(pub SXWord, i64); //Unsigned program address
crate::elf_define_type!(pub Half, u16); //Unsigned medium integer
crate::elf_define_type!(pub SWord, i32); //Unsigned integer
crate::elf_define_type!(pub XWord, u64); //Signed integer
crate::elf_define_type!(pub Word, u32); //Unsigned long integer
crate::elf_define_type!(pub Off, u64); //Signed long integer
crate::elf_define_type!(pub Addr, u64); //Unsigned small integer
