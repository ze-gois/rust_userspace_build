pub type pci8 = *const i8;

impl macros::traits::Bytes<crate::Origin, crate::Origin> for pci8 {
    const BYTES_SIZE: usize = core::mem::size_of::<Self>();
    fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
        if endianness {
            usize::to_le_bytes(*self as usize)
        } else {
            usize::to_be_bytes(*self as usize)
        }
    }

    fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self {
        if endianness {
            usize::from_le_bytes(bytes) as Self
        } else {
            usize::from_be_bytes(bytes) as Self
        }
    }
}

pub trait AType {
    fn from_pair(key: *mut usize, value: *mut u8) -> Self;
}

macros::enum_labeled! (
    pub Type,
    usize,
    "AT_TYPE",
    [
        [0;    Null;            usize;       AT_NULL;              "Null";            "End of vector"],
        [1;    Ignore;          usize;       AT_IGNORE;            "Ignore";          "Entry should be ignored"],
        [2;    ExecFD;          i32;         AT_EXECFD;              "ExecFD";          "File descriptor of program"],
        [3;    PHdr;            usize;       AT_PHDR;              "PHdr";            "Program headers for program"],
        [4;    PHEnt;           u16;         AT_PHENT;               "PHEnt";           "Size of program header entry"],
        [5;    PHNum;           u16;         ATPHNUM;               "PHNum";           "Number of program headers"],
        [6;    PageSz;          usize;       AT_PAGESZ;            "PageSz";          "System page size"],
        [7;    Base;            usize;       AT_BASE;              "Base";            "Base address of interpreter"],
        [8;    Flags;           u32;         AT_FLAGS;               "Flags";           "Flags"],
        [9;    Entry;           usize;       AT_ENTRY;             "Entry";           "Entry point of program"],
        [10;   NotELF;          i32;         AT_NOTELF;              "NotELF";          "Program is not ELF"],
        [11;   UID;             u32;         AT_UID;                 "UID";             "Real uid"],
        [12;   EUID;            u32;         AT_EUID;                "EUID";            "Effective uid"],
        [13;   GID;             u32;         AT_GID;                 "GID";             "Real gid"],
        [14;   EGID;            u32;         AT_EGID;                "EGID";            "Effective gid"],
        [15;   Platform;        pci8;        AT_PLATFORM;            "Platform";        "String identifying CPU for optimizations"],
        [16;   HwCap;           u64;         AT_HWCAP;               "HwCap";           "Arch dependent CPU capabilities hints"],
        [17;   ClkTck;          usize;       AT_CLKTCK;            "ClkTck";          "Frequency at which times() increments"],
        [23;   Secure;          i32;         AT_SECURE;              "Secure";          "Secure mode boolean"],
        [24;   BasePlatform;    pci8;        AT_BASE_PLATFORM;       "BasePlatform";    "Real platform string"],
        [25;   Random;          usize;       AT_RANDOM;            "Random";          "Address of 16 random bytes"],
        [26;   HwCap2;          u64;         AT_HWCAP2;              "HwCap2";          "Extension of AT_HWCAP"],
        [27;   RSeqFeatureSize; usize;       AT_RSEQ_FEATURE_SIZE; "RSeqFeatureSize"; "rseq supported feature size"],
        [28;   RSeqAlign;       usize;       AT_RSEQ_ALIGN;        "RSeqAlign";       "rseq allocation alignment"],
        [29;   HwCap3;          u64;         AT_HWCAP3;              "HwCap3";          "Extension of AT_HWCAP"],
        [30;   HwCap4;          u64;         AT_HWCAP4;              "HwCap4";          "Extension of AT_HWCAP"],
        [31;   ExecFn;          pci8;        AT_EXECFN;              "ExecFn";          "Filename of program"],
        [32;   SysInfo;         usize;       AT_SYSINFO;           "SysInfo";         "System info, x86 specific"],
        [33;   SysInfoEhdr;     usize;       AT_SYSINFO_EHDR;      "SysInfoEhdr";     "System info ELF header, x86 specific"],
        [51;   MinSigStackSz;   usize;       AT_MINSIGSTKSZ;       "MinSigStackSz";   "Minimal stack size for signal delivery"],
    ]
);
