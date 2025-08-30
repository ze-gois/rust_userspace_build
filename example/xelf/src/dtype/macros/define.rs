#[macro_export]
macro_rules! dtype_define {
    ($(#[$meta:meta])* $vis:vis $name:ident, $inner:ty) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        $vis struct $name(pub $inner);

        impl ELFType for $name {
            type Inner = $inner;
            const SIZE_BYTES: usize = core::mem::size_of::<$inner>();
            const SIZE_BITS: usize = Self::SIZE_BYTES * 8;
        }

        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}",self.0)?;
                Ok(())
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "0x{:#x}",self.0)?;
                Ok(())
            }
        }

        trait_implement_partial_eq!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        trait_implement_from!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
    }
}
