// use crate::dtype::Word as T;

macros::labeled_enum!(
    Type,
    isize,
    "HeaderType",
    [
        (Null, 0, "Unused", "d"),
        (ProgBits, 1, "Program-defined information", "n"),
        (SymTab, 2, "Linker symbol table", "e"),
        (StrTab, 3, "String table", "e"),
        (Rela, 4, "“Rela” relocation", "n"),
        (Hash, 5, "Symbol hash table", "e"),
        (Dynamic, 6, "Dynamic linking tables", "s"),
        (Note, 7, "Note information", "n"),
        (NoBits, 8, "Uninitialized space", "e"),
        (Rel, 9, "“Rel” relocation", "n"),
        (ShLib, 10, "Reserved", "d"),
        (DynSym, 11, "Dynamic loader symbol table", "e"),
        (
            LoProc,
            0x70000000,
            "Environment-specific use lower bound",
            "d"
        ),
        (
            HiProc,
            0x7FFFFFFF,
            "Environment-specific use upper bound",
            "d"
        ),
        (
            LoUser,
            0x80000000,
            "Processor-specific use lower bound",
            "d"
        ),
        (
            HiUser,
            0xFFFFFFFF,
            "Processor-specific use upper bound",
            "d"
        ),
        (Undefined, 12, "Undefined", "d"),
    ]
);

// #[derive(Clone, Copy, PartialEq, Eq)]
// #[repr(C)]
// pub enum Type {
//     /// Unused
//     Null = 0,
//     /// Program-defined information
//     ProgBits = 1,
//     /// Linker symbol table
//     SymTab = 2,
//     /// String table
//     StrTab = 3,
//     /// “Rela” relocation
//     Rela = 4,
//     /// Symbol hash table
//     Hash = 5,
//     /// Dynamic linking tables
//     Dynamic = 6,
//     /// Note information
//     Note = 7,
//     /// Uninitialized space
//     NoBits = 8,
//     /// “Rel” relocation
//     Rel = 9,
//     /// Reserved
//     ShLib = 10,
//     /// Dynamic loader symbol table
//     DynSym = 11,
//     /// Environment-specific use lower bound
//     LoProc = 0x70000000,
//     /// Environment-specific use upper bound
//     HiProc = 0x7FFFFFFF,
//     /// Processor-specific use lower bound
//     LoUser = 0x80000000,
//     /// Processor-specific use upper bound
//     HiUser = 0xFFFFFFFF,
//     /// Undefined
//     Undefined = 12,
// }

// impl Type {
//     pub fn from(i: T) -> Self {
//         match i {
//             0 => Self::Null,
//             1 => Self::ProgBits,
//             2 => Self::SymTab,
//             3 => Self::StrTab,
//             4 => Self::Rela,
//             5 => Self::Hash,
//             6 => Self::Dynamic,
//             7 => Self::Note,
//             8 => Self::NoBits,
//             9 => Self::Rel,
//             10 => Self::ShLib,
//             11 => Self::DynSym,
//             0x70000000 => Self::LoProc,
//             0x7FFFFFFF => Self::HiProc,
//             0x80000000 => Self::LoUser,
//             0xFFFFFFFF => Self::HiUser,
//             _ => Self::Undefined,
//         }
//     }

//     pub fn to(&self) -> T {
//         match self {
//             Self::Null => 0,
//             Self::ProgBits => 1,
//             Self::SymTab => 2,
//             Self::StrTab => 3,
//             Self::Rela => 4,
//             Self::Hash => 5,
//             Self::Dynamic => 6,
//             Self::Note => 7,
//             Self::NoBits => 8,
//             Self::Rel => 9,
//             Self::ShLib => 10,
//             Self::DynSym => 11,
//             Self::LoProc => 0x70000000,
//             Self::HiProc => 0x7FFFFFFF,
//             Self::LoUser => 0x80000000,
//             Self::HiUser => 0xFFFFFFFF,
//             Self::Undefined => 12,
//         }
//     }

//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Self::Null => "Null (Unused)",
//             Self::ProgBits => "ProgBits (Program-defined information)",
//             Self::SymTab => "SymTab (Linker symbol table)",
//             Self::StrTab => "StrTab (String table)",
//             Self::Rela => "“Rela (Rela” relocation)",
//             Self::Hash => "Hash (Symbol hash table)",
//             Self::Dynamic => "Dynamic (Dynamic linking tables)",
//             Self::Note => "Note (Note information)",
//             Self::NoBits => "NoBits (Uninitialized space)",
//             Self::Rel => "“Rel (Rel” relocation)",
//             Self::ShLib => "ShLib (Reserved)",
//             Self::DynSym => "DynSym (Dynamic loader symbol table)",
//             Self::LoProc => "LoProc (Environment-specific use lower bound)",
//             Self::HiProc => "HiProc (Environment-specific use upper bound)",
//             Self::LoUser => "LoUser (Processor-specific use lower bound)",
//             Self::HiUser => "HiUser (Processor-specific use upper bound)",
//             Self::Undefined => "Undefined (Undefined)",
//         }
//     }
// }

// use core::fmt::{Debug, Display, Formatter, Result};

// type E = Type;

// impl Display for E {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{}", self.as_str())
//     }
// }

// impl Debug for E {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{}", self.as_str())
//     }
// }
