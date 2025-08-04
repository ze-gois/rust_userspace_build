#[macro_export]
macro_rules! elf_define_type {
    ($(#[$meta:meta])* $vis:vis $name:ident, $inner:ty) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq)]
        $vis struct $name(pub $inner);

        impl $crate::dtype::ELFType for $name {
            type Inner = $inner;
            const SIZE_BYTES: usize = core::mem::size_of::<$inner>();
            const SIZE_BITS: usize = Self::SIZE_BYTES * 8;
        }

        impl $name {
            $vis fn new(value: $inner) -> Self {
                Self(value)
            }

            $vis fn from_bytes(bytes: [u8; Self::SIZE_BYTES], endianness: $crate::dtype::Endianness) -> Self {
                match endianness {
                    Endianness::TODO => {
                        $crate::info!("Endianness is TODO");
                        panic!("TODO");
                    },
                    Endianness::None => {
                        $crate::info!("Endianness is zeroed.");
                        panic!("none.")
                    },
                    Endianness::LSB => Self(<$inner>::from_le_bytes(bytes)),
                    Endianness::MSB => Self(<$inner>::from_be_bytes(bytes)),
                    Endianness::Number => {
                        $crate::info!("Endianness is invalid number.");
                        panic!("none.")
                    },
                    Endianness::Undefined => {
                        $crate::info!("Endianness is undefined.");
                        panic!("none.")
                    },
                }
            }

            $vis fn to_bytes(&self, endianness: $crate::dtype::Endianness) -> [u8; Self::SIZE_BYTES] {
                match endianness {
                    Endianness::TODO => {
                        $crate::info!("Endianness is TODO");
                        panic!("TODO");
                    },
                    Endianness::None => {
                        $crate::info!("Endianness is zeroed.");
                        panic!("none.")
                    },
                    Endianness::LSB => self.0.to_le_bytes(),
                    Endianness::MSB => self.0.to_be_bytes(),
                    Endianness::Number => {
                        $crate::info!("Endianness is invalid number.");
                        panic!("none.")
                    },
                    Endianness::Undefined => {
                        $crate::info!("Endianness is undefined.");
                        panic!("none.")
                    },
                }
            }

            $vis fn read(fd: isize, endianness: $crate::dtype::Endianness) -> Result<$name> {
                let mut bytes = [0u8; <$name>::SIZE_BYTES];
                const INNER_SIZE : isize = <$name>::SIZE_BYTES as isize;
                match syscall::read(fd, bytes.as_mut_ptr(), <$name>::SIZE_BYTES) {
                    Ok(INNER_SIZE) => {
                        let value = match endianness {
                            Endianness::TODO => {
                                $crate::info!("Endianness is TODO");
                                panic!("TODO");
                            },
                            Endianness::None => {
                                $crate::info!("Endianness is zeroed.");
                                panic!("none.")
                            },
                            Endianness::LSB => Self(<$inner>::from_le_bytes(bytes)),
                            Endianness::MSB => Self(<$inner>::from_be_bytes(bytes)),
                            Endianness::Number => {
                                $crate::info!("Endianness is invalid number.");
                                panic!("none.")
                            },
                            Endianness::Undefined => {
                                $crate::info!("Endianness is undefined.");
                                panic!("none.")
                            },
                        };
                        Ok($name::from(value))
                    },
                    Ok(e) => Err($crate::result::Error::DType(Error::ShorterData(e))),
                    Err(e) => Err($crate::result::Error::DType(Error::InvalidData(e.into()))),
                }
            }
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


        $crate::macros::impl_partial_eq_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
        $crate::macros::impl_from_for_type!($name, $inner, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);
    }
}
