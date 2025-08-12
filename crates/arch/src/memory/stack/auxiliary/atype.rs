mod x {
    #[allow(non_camel_case_types)]
    pub type mi8 = *const i8;
}

::macros::enum_typed!(Type, usize, "AT_TYPE", [crate::memory::stack::auxiliary::atype::x::mi8], [
//   discriminant variant       output             lambda              constat      acronym        description
    [0;           Null;         usize;       |p|{ *p as usize };     AT_NULL;        "Null";        "End of vector"],
    [1;           Ignore;       usize;       |p|{ *p as usize };     AT_IGNORE;      "Ignore";      "Entry should be ignored"],
    [2;           ExecFD;         i32;       |p|{ *p as i32   };     AT_EXECFD;      "ExecFD";      "File descriptor of program"],
    [3;           PHdr;         usize;       |p|{ *p as usize };     AT_PHDR;        "PHdr";        "Program headers for program"],
    [4;           PHEnt;          u16;       |p|{ *p as u16   };     AT_PHENT;       "PHEnt";       "Size of program header entry"],
    [5;           PHNum;          u16;       |p|{ *p as u16   };     AT_PHNUM;       "PHNum";       "Number of program headers"],
    [6;           PageSz;       usize;       |p|{ *p as usize };     AT_PAGESZ;      "PageSz";      "System page size"],
    [7;           Base;         usize;       |p|{ *p as usize };     AT_BASE;        "Base";        "Base address of interpreter"],
    [8;           Flags;          u32;       |p|{ *p as u32   };     AT_FLAGS;       "Flags";       "Flags"],
    [9;           Entry;        usize;       |p|{ *p as usize };     AT_ENTRY;       "Entry";       "Entry point of program"],
    [10;          NotELF;         i32;       |p|{ *p as i32   };     AT_NOTELF;      "NotELF";      "Program is not ELF"],
    [11;          UID;            u32;       |p|{ *p as u32   };     AT_UID;         "UID";         "Real uid"],
    [12;          EUID;           u32;       |p|{ *p as u32   };     AT_EUID;        "EUID";        "Effective uid"],
    [13;          GID;            u32;       |p|{ *p as u32   };     AT_GID;         "GID";         "Real gid"],
    [14;          EGID;           u32;       |p|{ *p as u32   };     AT_EGID;        "EGID";        "Effective gid"],
    [15;          Platform;       mi8;       |p|{ *p as mi8   };     AT_PLATFORM;    "at_platform"; "ABI fallback"],
    [17;          ClkTck;         u64;       |p|{ *p as u64   };     AT_CLKTCK;      "ClkTck";      "Frequency of times()"],
    [31;          ExecFn;       usize;       |p|{ *p as usize };     AT_EXECFN;      "ExecFn";      "ExecFn"],
    [32;          SysInfo;      usize;       |p|{ *p as usize };     AT_SYSINFO;     "SysInfo";     "SysInfo"],
    [33;          SysInfoEhdr;  usize;       |p|{ *p as usize };     AT_SYSINFOEHDR; "SysInfoEhdr"; "SysInfoEhdr"],
    [25;          Random;       usize;       |p|{ *p as usize };     AT_RANDOM;      "RandomBytes"; "Random 16 bytes"],
    [16;          HwCap;          u64;       |p|{ *p as u64   };     AT_HWCAP;       "at_hwcap";    "CPU Bitmask Capacity"],
    [26;          HwCap2;         u64;       |p|{ *p as u64   };     AT_HWCAP2;      "at_hwcap2";   "AT_HWCAP Extension"],
    [23;          Secure;         i32;       |p|{ *p as i32   };     AT_SECURE;      "at_secure";   "Secure mode flag"],
    [0xFFFFFFFF;  DontCare;     usize;       |p|{ *p as usize };     DONTCARE;       "DontCare";    "DontCare"],
]);

// ::macros::enum_typed!(Type, usize, "AT_TYPE", [
// //   discriminant variant       output             lambda              constat      acronym        description
//     [0;           Null;         usize;       |p|{ *p as usize };     AT_NULL;        "Null";        "End of vector"],
//     [1;           Ignore;       usize;       |p|{ *p as usize };     AT_IGNORE;      "Ignore";      "Entry should be ignored"],
//     [2;           ExecFD;         i32;       |p|{ *p as i32   };     AT_EXECFD;      "ExecFD";      "File descriptor of program"],
//     [3;           PHdr;         usize;       |p|{ *p as usize };     AT_PHDR;        "PHdr";        "Program headers for program"],
//     [4;           PHEnt;          u16;       |p|{ *p as u16   };     AT_PHENT;       "PHEnt";       "Size of program header entry"],
//     [5;           PHNum;          u16;       |p|{ *p as u16   };     AT_PHNUM;       "PHNum";       "Number of program headers"],
//     [6;           PageSz;       usize;       |p|{ *p as usize };     AT_PAGESZ;      "PageSz";      "System page size"],
//     [7;           Base;         usize;       |p|{ *p as usize };     AT_BASE;        "Base";        "Base address of interpreter"],
//     [8;           Flags;          u32;       |p|{ *p as u32   };     AT_FLAGS;       "Flags";       "Flags"],
//     [9;           Entry;        usize;       |p|{ *p as usize };     AT_ENTRY;       "Entry";       "Entry point of program"],
//     [10;          NotELF;         i32;       |p|{ *p as i32   };     AT_NOTELF;      "NotELF";      "Program is not ELF"],
//     [11;          UID;            u32;       |p|{ *p as u32   };     AT_UID;         "UID";         "Real uid"],
//     [12;          EUID;           u32;       |p|{ *p as u32   };     AT_EUID;        "EUID";        "Effective uid"],
//     [13;          GID;            u32;       |p|{ *p as u32   };     AT_GID;         "GID";         "Real gid"],
//     [14;          EGID;           u32;       |p|{ *p as u32   };     AT_EGID;        "EGID";        "Effective gid"],
//     [15;          Platform;  core::ffi::CStr; |p|{ core::ffi::CStr::from_pointer(p as *mut i8)}; AT_PLATFORM; "at_platform"; "ABI fallback"],
//     [17;          ClkTck;         u64;       |p|{ *p as u64   };     AT_CLKTCK;      "ClkTck";      "Frequency of times()"],
//     [31;          ExecFn;       usize;       |p|{ *p as usize };     AT_EXECFN;      "ExecFn";      "ExecFn"],
//     [32;          SysInfo;      usize;       |p|{ *p as usize };     AT_SYSINFO;     "SysInfo";     "SysInfo"],
//     [33;          SysInfoEhdr;  usize;       |p|{ *p as usize };     AT_SYSINFOEHDR; "SysInfoEhdr"; "SysInfoEhdr"],
//     [25;          Random;       usize;       |p|{ *p as usize };     AT_RANDOM;      "RandomBytes"; "Random 16 bytes"],
//     [16;          HwCap;          u64;       |p|{ *p as u64   };     AT_HWCAP;       "at_hwcap";    "CPU Bitmask Capacity"],
//     [26;          HwCap2;         u64;       |p|{ *p as u64   };     AT_HWCAP2;      "at_hwcap2";   "AT_HWCAP Extension"],
//     [23;          Secure;         i32;       |p|{ *p as i32   };     AT_SECURE;      "at_secure";   "Secure mode flag"],
//     [0xFFFFFFFF;  DontCare;     usize;       |p|{ *p as usize };     DONTCARE;       "DontCare";    "DontCare"],
// ]);

// at_random (25)	*const u8 → [u8; 16]	Endereço para 16 bytes aleatórios, usado para ASLR, stack canaries, seeds. Importante para replicar ambiente seguro.
// at_platform (15)	*const u8 (C string)	Nome da plataforma (ex.: "x86_64"). Pode guiar otimizações ou fallback de ABI.
// at_hwcap (16)	u64	Bitmask de capacidades da CPU detectadas no boot (SSE, AVX, etc.).
// at_hwcap2 (26)	u64	Extensão de AT_HWCAP com flags extras.
// at_secure (23)	i32	Indica se o binário está rodando em modo seguro (setuid, setgid, secure exec). Importante para decidir se ignora variáveis de ambiente.

// at_phdr	*const libc::Elf_Phdr (ou usize como endereço)	Endereço da tabela de program headers
// at_execfn	*const u8 (C string)	Nome do executável
// at_sysinfo	usize	Endereço da função sysinfo (vsyscall)
// at_sysinfo_ehdr	usize	Endereço do cabeçalho ELF para vDSO
// at_dontcare	usize	Valor irrelevante
