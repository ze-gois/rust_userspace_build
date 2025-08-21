#[macro_export]
macro_rules! do_enum {
    ($enum_module:ident, $enum_identifier:ident, $enum_discriminant_type:ty, [$([$variant_discriminant:expr,$variant_identifier:ident,$variant_type:ty]),* $(,)? ]) => {
        #[derive(Debug, Clone, Copy)]
        enum $enum_identifier {
            $($variant_identifier($variant_type)),*
        }

        impl XElfSize for $enum_identifier {
            const XELF_SIZE : usize = core::mem::size_of::<$enum_discriminant_type>()+ max_const!($(<$variant_type>::XELF_SIZE),*);
        }

        impl $enum_identifier {
            pub fn discriminant(&self) -> usize {
                match self {
                    $($enum_identifier::$variant_identifier(_) => $variant_discriminant),*
                }
            }

            pub fn to_bytes(&self, endianness: bool) -> [u8;<$enum_identifier>::XELF_SIZE] {
                let mut bytes = [0u8;<$enum_identifier>::XELF_SIZE];

                match self {
                    $(
                        $enum_identifier::$variant_identifier(payload) => {
                            let discriminant = self.discriminant();

                            let mut o = 0;
                            bytes[o..(o+<$enum_discriminant_type>::XELF_SIZE)].copy_from_slice(
                                &if endianness {
                                    <$enum_discriminant_type>::to_le_bytes(discriminant)
                                } else {
                                    <$enum_discriminant_type>::to_be_bytes(discriminant)
                                }
                            );
                            o = o + <$enum_discriminant_type>::XELF_SIZE;
                            bytes[o..(o+<$variant_type>::XELF_SIZE)].copy_from_slice(
                                &if endianness {
                                    <$variant_type>::to_le_bytes(payload)
                                } else {
                                    <$variant_type>::to_be_bytes(payload)
                                }
                            );
                            bytes
                        }
                    ),*
                }
            }

            pub fn to_le_bytes(&self) -> [u8;<$enum_identifier>::XELF_SIZE] {
                self.to_bytes(true)
            }

            pub fn to_be_bytes(&self) -> [u8;<$enum_identifier>::XELF_SIZE] {
                self.to_bytes(false)
            }


            pub fn from_bytes(bytes: [u8;<$enum_identifier>::XELF_SIZE], endianness: bool) -> $enum_identifier {
                let mut o = 0;
                let mut discriminant_bytes = [0u8; <$enum_discriminant_type>::XELF_SIZE];
                discriminant_bytes.copy_from_slice(&bytes[o..(o+<$enum_discriminant_type>::XELF_SIZE)]);
                let discriminant = if endianness {
                    <$enum_discriminant_type>::from_le_bytes(discriminant_bytes)
                } else {
                    <$enum_discriminant_type>::from_be_bytes(discriminant_bytes)
                };
                o = o + <$enum_discriminant_type>::XELF_SIZE;
                match discriminant {
                    $(
                        $variant_discriminant => {
                            $enum_identifier::$variant_identifier({
                                let mut payload = [0u8; <$variant_type>::XELF_SIZE];
                                payload.copy_from_slice(&bytes[o..(o+<$variant_type>::XELF_SIZE)]);
                                if endianness {
                                    <$variant_type>::from_le_bytes(payload)
                                } else {
                                    <$variant_type>::from_be_bytes(payload)
                                }
                            })
                        },
                    )*
                    _ => unreachable!()
                }

            }

            pub fn from_le_bytes(bytes: [u8;<$enum_identifier>::XELF_SIZE]) -> $enum_identifier {
                $enum_identifier::from_bytes(bytes, true)
            }

            pub fn from_be_bytes(bytes: [u8;<$enum_identifier>::XELF_SIZE]) -> $enum_identifier {
                $enum_identifier::from_bytes(bytes, false)
            }
        }
    };
}
