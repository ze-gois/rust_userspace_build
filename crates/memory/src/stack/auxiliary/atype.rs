pub type pci8 = *const i8;
pub type pcu = *const usize;
pub type pair = (pcu, pci8);

macro_rules! impl_pointer {
    ($($pointer_type:ty),*) => {
        $(
            impl macros::traits::Bytes<crate::Origin, crate::Origin> for $pointer_type {
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
        )*
    };
}

impl_pointer!(pci8, pcu);

impl macros::traits::Bytes<crate::Origin, crate::Origin> for pair {
    const BYTES_SIZE: usize =
        <pcu as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE
            + <pci8 as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
    fn to_bytes(
        &self,
        endianness: bool,
    ) -> [u8; <Self as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE] {
        let mut pair_bytes =
            [0u8; <Self as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];

        if endianness {
            let mut o = 0;
            let mut l = <pcu as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            pair_bytes[o..l].copy_from_slice(&self.0.to_le_bytes());
            o = l;
            l = l + <pci8 as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            pair_bytes[o..l].copy_from_slice(&self.1.to_le_bytes());
        } else {
            let mut o = 0;
            let mut l = <pcu as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            pair_bytes[o..l].copy_from_slice(&self.0.to_be_bytes());
            o = l;
            l = l + <pci8 as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;
            pair_bytes[o..l].copy_from_slice(&self.1.to_be_bytes());
        }

        pair_bytes
    }

    fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self {
        let mut left_bytes =
            [0u8; <pcu as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];
        let mut right_bytes =
            [0u8; <pci8 as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE];

        let mut o = 0;
        let mut l = <pcu as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

        left_bytes.copy_from_slice(&bytes[o..l]);

        o = l;
        l = l + <pci8 as macros::traits::Bytes<crate::Origin, crate::Origin>>::BYTES_SIZE;

        right_bytes.copy_from_slice(&bytes[o..l]);

        if endianness {
            let left = <pcu as macros::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(
                left_bytes,
            );
            let right =
                <pci8 as macros::traits::Bytes<crate::Origin, crate::Origin>>::from_le_bytes(
                    right_bytes,
                );
            (left, right)
        } else {
            let left = <pcu as macros::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(
                left_bytes,
            );
            let right =
                <pci8 as macros::traits::Bytes<crate::Origin, crate::Origin>>::from_be_bytes(
                    right_bytes,
                );
            (left, right)
        }
    }
}

pub trait AType {
    fn from_pair(etype: *const usize, p: *const u8) -> Self;
    fn is_null(&self) -> bool;
}

macro_rules! bring_atype {
    (
        $enum_vis:vis $enum_identifier:ident,
        $enum_discriminant_type:ty,
        $enum_label:expr,
        [
            $(
                [
                    $variant_discriminant:expr;
                    $variant_identifier:ident;
                    $variant_type:ty;
                    $variant_const_identifier:ident;
                    $variant_acronym:expr;
                    $variant_description:expr
                ]
            ),* $(,)?
        ]
    ) => {
        macros::enum_labeled!(
            $enum_vis $enum_identifier,
            $enum_discriminant_type,
            $enum_label,
            [
                $(
                    [
                        $variant_discriminant;
                        $variant_identifier;
                        $variant_type;
                        $variant_const_identifier;
                        $variant_acronym;
                        $variant_description
                    ]
                ),*
            ]
        );

        {
            macros::enum_labeled!(
                enum_intermediario,
                $enum_discriminant_type,
                $enum_label,
                [
                    $(
                        [
                            $variant_discriminant;
                            $variant_identifier;
                            ();
                            $variant_const_identifier;
                            $variant_acronym;
                            $variant_description
                        ]
                    ),*
                ]
            );

            trait FromDiscriminant {
                fn from_discriminant(discriminant: $enum_discriminant_type) -> Self;
            }

            impl FromDiscriminant for enum_intermediario {
                fn from_discriminant(discriminant : $enum_discriminant_type ) -> enum_intermediario {
                    match discriminant {
                        $( $variant_discriminant => enum_intermediario::$variant_identifier(()), )*
                        _ => enum_intermediario::TODO(())
                    }
                }
            }
        }

        impl AType for $enum_identifier {
            fn from_pair(etype: *const usize, p: *const u8) -> Self {
                match enum_intermediario::from_discriminant(unsafe { *etype }) {
                    $( enum_intermediario::$variant_identifier(()) => $enum_identifier::$variant_identifier( unsafe { p as $variant_type } ),)*
                    _ => $enum_identifier::TODO({
                        let v :pair =(etype, p);
                        v
                    })
                }
            }

            fn is_null(&self) -> bool {
                match self {
                    $enum_identifier::Null(0) => true,
                    _ => false,
                }
            }
        }
    };
}

bring_atype! (
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
        [99;   TODO;            pair;        AT_TODO;               "Pkmna";          "Pokemon not found"]
    ]
);

// impl EnumTyped {

//     pub fn to_kv(&self) -> ($variant, *const u8) {
//         match *self {
//             $(EnumTyped::$identifier(v) => (($enum_identifier::$identifier).to(), v as *const u8),)*
//             EnumTyped::TODO(id) => ($enum_identifier::TODO.to(), id.0 as *const u8),
//         }
//     }

//     pub fn to_k(&self) -> $enum_identifier {
//         match *self {
//             $(EnumTyped::$identifier(_) => $enum_identifier::$identifier,)*
//             EnumTyped::TODO(id) => $enum_identifier::TODO,
//         }
//     }

//     pub fn is_null(&self) -> bool {

//     }
// }
