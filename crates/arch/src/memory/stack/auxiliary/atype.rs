pub mod macro_types {
    #[allow(non_camel_case_types)]
    pub type mi8 = *const i8;
    pub type mus = *const u8;
}

::macros::enum_typed!(Type, usize, "AT_TYPE", [crate::memory::stack::auxiliary::atype::macro_types::mi8; crate::memory::stack::auxiliary::atype::macro_types::mus], [
//   discriminant variant       output             lambda              constat      acronym        description
    [0;           Null;         usize;       |p : mus|{ *p as usize };     AT_NULL;        "Null";        "End of vector"],
    [1;           Ignore;       usize;       |p : mus|{ *p as usize };     AT_IGNORE;      "Ignore";      "Entry should be ignored"],
    [2;           ExecFD;         i32;       |p : mus|{ *p as i32   };     AT_EXECFD;      "ExecFD";      "File descriptor of program"],
    [3;           PHdr;         usize;       |p : mus|{ *p as usize };     AT_PHDR;        "PHdr";        "Program headers for program"],
    [4;           PHEnt;          u16;       |p : mus|{ *p as u16   };     AT_PHENT;       "PHEnt";       "Size of program header entry"],
    [5;           PHNum;          u16;       |p : mus|{ *p as u16   };     AT_PHNUM;       "PHNum";       "Number of program headers"],
    [6;           PageSz;       usize;       |p : mus|{ *p as usize };     AT_PAGESZ;      "PageSz";      "System page size"],
    [7;           Base;         usize;       |p : mus|{ *p as usize };     AT_BASE;        "Base";        "Base address of interpreter"],
    [8;           Flags;          u32;       |p : mus|{ *p as u32   };     AT_FLAGS;       "Flags";       "Flags"],
    [9;           Entry;        usize;       |p : mus|{ *p as usize };     AT_ENTRY;       "Entry";       "Entry point of program"],
    [10;          NotELF;         i32;       |p : mus|{ *p as i32   };     AT_NOTELF;      "NotELF";      "Program is not ELF"],
    [11;          UID;            u32;       |p : mus|{ *p as u32   };     AT_UID;         "UID";         "Real uid"],
    [12;          EUID;           u32;       |p : mus|{ *p as u32   };     AT_EUID;        "EUID";        "Effective uid"],
    [13;          GID;            u32;       |p : mus|{ *p as u32   };     AT_GID;         "GID";         "Real gid"],
    [14;          EGID;           u32;       |p : mus|{ *p as u32   };     AT_EGID;        "EGID";        "Effective gid"],
    [15;          Platform;       mi8;       |p : mus|{ *p as mi8   };     AT_PLATFORM;    "at_platform"; "ABI fallback"],
    [17;          ClkTck;         u64;       |p : mus|{ *p as u64   };     AT_CLKTCK;      "ClkTck";      "Frequency of times()"],
    [31;          ExecFn;       usize;       |p : mus|{ *p as usize };     AT_EXECFN;      "ExecFn";      "ExecFn"],
    [32;          SysInfo;      usize;       |p : mus|{ *p as usize };     AT_SYSINFO;     "SysInfo";     "SysInfo"],
    [33;          SysInfoEhdr;  usize;       |p : mus|{ *p as usize };     AT_SYSINFOEHDR; "SysInfoEhdr"; "SysInfoEhdr"],
    [25;          Random;       usize;       |p : mus|{ *p as usize };     AT_RANDOM;      "RandomBytes"; "Random 16 bytes"],
    [16;          HwCap;          u64;       |p : mus|{ *p as u64   };     AT_HWCAP;       "at_hwcap";    "CPU Bitmask Capacity"],
    [26;          HwCap2;         u64;       |p : mus|{ *p as u64   };     AT_HWCAP2;      "at_hwcap2";   "AT_HWCAP Extension"],
    [23;          Secure;         i32;       |p : mus|{ *p as i32   };     AT_SECURE;      "at_secure";   "Secure mode flag"],
    [0xFFFFFFFF;  DontCare;     usize;       |p : mus|{ *p as usize };     DONTCARE;       "DontCare";    "DontCare"],
]);
