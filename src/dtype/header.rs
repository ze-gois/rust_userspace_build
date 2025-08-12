// ELF Header (common for 32/64-bit, but fields vary in size)
#[repr(C, packed)]
struct ElfHeader {
    e_ident: [u8; EI_NIDENT], // Identification bytes
                              // Remaining fields are read dynamically based on class/endians
}
