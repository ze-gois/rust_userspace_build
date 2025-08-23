pub mod macro_types {
    #[allow(non_camel_case_types)]
    pub type Mi8 = *const i8;
    pub type Mus = *const u8;
}

::macros::enum_typed!(Type; usize; "AT_TYPE"; crate::memory::stack::auxiliary::atype::macro_types; [
    [0;    Null;            usize;  |p: Mus| { *p as usize }; AT_NULL;              "Null";            "End of vector"],
    [1;    Ignore;          usize;  |p: Mus| { *p as usize }; AT_IGNORE;            "Ignore";          "Entry should be ignored"],
    [2;    ExecFD;          i32;    |p: Mus| { *p as i32 };   AT_EXECFD;            "ExecFD";          "File descriptor of program"],
    [3;    PHdr;            usize;  |p: Mus| { *p as usize }; AT_PHDR;              "PHdr";            "Program headers for program"],
    [4;    PHEnt;           u16;    |p: Mus| { *p as u16 };   AT_PHENT;             "PHEnt";           "Size of program header entry"],
    [5;    PHNum;           u16;    |p: Mus| { *p as u16 };   ATPHNUM;             "PHNum";           "Number of program headers"],
    [6;    PageSz;          usize;  |p: Mus| { *p as usize }; AT_PAGESZ;            "PageSz";          "System page size"],
    [7;    Base;            usize;  |p: Mus| { *p as usize }; AT_BASE;              "Base";            "Base address of interpreter"],
    [8;    Flags;           u32;    |p: Mus| { *p as u32 };   AT_FLAGS;             "Flags";           "Flags"],
    [9;    Entry;           usize;  |p: Mus| { *p as usize }; AT_ENTRY;             "Entry";           "Entry point of program"],
    [10;   NotELF;          i32;    |p: Mus| { *p as i32 };   AT_NOTELF;            "NotELF";          "Program is not ELF"],
    [11;   UID;             u32;    |p: Mus| { *p as u32 };   AT_UID;               "UID";             "Real uid"],
    [12;   EUID;            u32;    |p: Mus| { *p as u32 };   AT_EUID;              "EUID";            "Effective uid"],
    [13;   GID;             u32;    |p: Mus| { *p as u32 };   AT_GID;               "GID";             "Real gid"],
    [14;   EGID;            u32;    |p: Mus| { *p as u32 };   AT_EGID;              "EGID";            "Effective gid"],
    [15;   Platform;        Mi8;    |p: Mus| { *p as Mi8 };   AT_PLATFORM;          "Platform";        "String identifying CPU for optimizations"],
    [16;   HwCap;           u64;    |p: Mus| { *p as u64 };   AT_HWCAP;             "HwCap";           "Arch dependent CPU capabilities hints"],
    [17;   ClkTck;          usize;  |p: Mus| { *p as usize }; AT_CLKTCK;            "ClkTck";          "Frequency at which times() increments"],
    [23;   Secure;          i32;    |p: Mus| { *p as i32 };   AT_SECURE;            "Secure";          "Secure mode boolean"],
    [24;   BasePlatform;    Mi8;    |p: Mus| { *p as Mi8 };   AT_BASE_PLATFORM;     "BasePlatform";    "Real platform string"],
    [25;   Random;          usize;  |p: Mus| { *p as usize }; AT_RANDOM;            "Random";          "Address of 16 random bytes"],
    [26;   HwCap2;          u64;    |p: Mus| { *p as u64 };   AT_HWCAP2;            "HwCap2";          "Extension of AT_HWCAP"],
    [27;   RSeqFeatureSize; usize;  |p: Mus| { *p as usize }; AT_RSEQ_FEATURE_SIZE; "RSeqFeatureSize"; "rseq supported feature size"],
    [28;   RSeqAlign;       usize;  |p: Mus| { *p as usize }; AT_RSEQ_ALIGN;        "RSeqAlign";       "rseq allocation alignment"],
    [29;   HwCap3;          u64;    |p: Mus| { *p as u64 };   AT_HWCAP3;            "HwCap3";          "Extension of AT_HWCAP"],
    [30;   HwCap4;          u64;    |p: Mus| { *p as u64 };   AT_HWCAP4;            "HwCap4";          "Extension of AT_HWCAP"],
    [31;   ExecFn;          Mi8;    |p: Mus| { *p as Mi8 };   AT_EXECFN;            "ExecFn";          "Filename of program"],
    [32;   SysInfo;         usize;  |p: Mus| { *p as usize }; AT_SYSINFO;           "SysInfo";         "System info, x86 specific"],
    [33;   SysInfoEhdr;     usize;  |p: Mus| { *p as usize }; AT_SYSINFO_EHDR;      "SysInfoEhdr";     "System info ELF header, x86 specific"],
    [51;   MinSigStackSz;   usize;  |p: Mus| { *p as usize }; AT_MINSIGSTKSZ;       "MinSigStackSz";   "Minimal stack size for signal delivery"],
]);
