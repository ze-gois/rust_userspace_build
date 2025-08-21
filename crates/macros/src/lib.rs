#![no_std]

#[macro_use]
pub mod chain;

#[macro_use]
pub mod r#enum;

#[macro_use]
pub mod result;

#[macro_use]
pub mod r#struct;

// #[macro_use]
// pub mod pointer;

macro_rules! max_const {
    // caso base: apenas um argumento
    ($x:expr) => { $x };
    // caso recursivo: pelo menos dois
    ($x:expr, $($rest:expr),+) => {
        {
            let y = max_const!($($rest),+);
            if $x > y { $x } else { y }
        }
    };
}

pub trait XElfSize {
    const XELF_SIZE: usize;
}

macro_rules! xelf_size_as_mem_size {
    ($($t:ty),*) => {
       $(
           impl XElfSize for $t {
                const XELF_SIZE : usize = core::mem::size_of::<$t>();
           }
       )*
    };
}

xelf_size_as_mem_size!(
    bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

#[macro_export]
macro_rules! impl_partial_eq_for_type {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            impl core::cmp::PartialEq<$t> for $name {
                fn eq(&self, other: &$t) -> bool {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        self.0 == *other as $inner
                    } else {
                        false
                    }
                }
            }

            impl core::cmp::PartialEq<$name> for $t {
                fn eq(&self, other: &$name) -> bool {
                    if (*self as u128) <= <$inner>::MAX as u128 {
                        *self as $inner == other.0
                    } else {
                        false
                    }
                }
            }

            impl core::cmp::PartialOrd<$t> for $name {
                fn partial_cmp(&self, other: &$t) -> Option<core::cmp::Ordering> {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        Some(self.0.cmp(&(*other as $inner)))
                    } else {
                        None
                    }
                }
            }

            impl core::cmp::PartialOrd<$name> for $t {
                fn partial_cmp(&self, other: &$name) -> Option<core::cmp::Ordering> {
                    if (*self as u128) <= <$inner>::MAX as u128 {
                        Some((*self as $inner).cmp(&other.0))
                    } else {
                        None
                    }
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! impl_from_for_type {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            impl From<$name> for $t {
                fn from(val: $name) -> $t {
                    val.0 as $t
                }
            }

            impl From<$t> for $name {
                fn from(v: $t) -> Self {
                    Self(v as $inner)
                }
            }
        )*
    }
}
