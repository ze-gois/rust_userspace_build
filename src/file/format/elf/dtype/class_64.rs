use super::ELFType;

file_format_elf_dtype_define!(pub Null, u8); //Unsigned file offset
file_format_elf_dtype_define!(pub UChar, u8); //Unsigned file offset
file_format_elf_dtype_define!(pub SXWord, i64); //Unsigned program address
file_format_elf_dtype_define!(pub Half, u16); //Unsigned medium integer
file_format_elf_dtype_define!(pub SWord, i32); //Unsigned integer
file_format_elf_dtype_define!(pub XWord, u64); //Signed integer
file_format_elf_dtype_define!(pub Word, u32); //Unsigned long integer
file_format_elf_dtype_define!(pub Off, u64); //Signed long integer
file_format_elf_dtype_define!(pub Addr, u64); //Unsigned small integer

file_format_elf_dtype_impl!(Null, UChar, SXWord, Half, SWord, XWord, Word, Off, Addr);
