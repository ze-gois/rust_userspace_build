#[macro_export]
macro_rules! r#enum {
    ($enum_vis:vis $enum_identifier:ident, $enum_discriminant_type:ty, [$([$variant_discriminant:expr,$variant_identifier:ident,$variant_type:ty]),* $(,)? ]) => {
        #[derive(Debug, Clone, Copy)]
        $enum_vis enum $enum_identifier {
            $($variant_identifier($variant_type)),*
        }

        impl $enum_identifier {
            pub fn discriminant(&self) -> usize {
                match self {
                    $($enum_identifier::$variant_identifier(_) => $variant_discriminant),*
                }
            }
        }

        impl $crate::macros::traits::Bytes for $enum_identifier {
            const BYTES_SIZE : usize = core::mem::size_of::<$enum_discriminant_type>()+ ::macros::expressions_upperbound!($(<$variant_type>::BYTES_SIZE),*);
            pub fn to_bytes(&self, endianness: bool) -> [u8;Self::BYTES_SIZE] {
                let mut bytes = [0u8;Self::BYTES_SIZE];

                match self {
                    $(
                        Self::$variant_identifier(payload) => {
                            let discriminant = self.discriminant();

                            let mut o = 0;
                            bytes[o..(o+<$enum_discriminant_type>::BYTES_SIZE)].copy_from_slice(
                                &if endianness {
                                    <$enum_discriminant_type>::to_le_bytes(discriminant)
                                } else {
                                    <$enum_discriminant_type>::to_be_bytes(discriminant)
                                }
                            );
                            o = o + <$enum_discriminant_type>::BYTES_SIZE;
                            bytes[o..(o+<$variant_type>::BYTES_SIZE)].copy_from_slice(
                                &if endianness {
                                    <$variant_type>::to_le_bytes(*payload)
                                } else {
                                    <$variant_type>::to_be_bytes(*payload)
                                }
                            );
                            bytes
                        }
                    ),*
                }
            }

            pub fn from_bytes(bytes: [u8;Self::BYTES_SIZE], endianness: bool) -> Self {
                let mut o = 0;
                let mut discriminant_bytes = [0u8; <$enum_discriminant_type>::BYTES_SIZE];
                discriminant_bytes.copy_from_slice(&bytes[o..(o+<$enum_discriminant_type>::BYTES_SIZE)]);
                let discriminant = if endianness {
                    <$enum_discriminant_type>::from_le_bytes(discriminant_bytes)
                } else {
                    <$enum_discriminant_type>::from_be_bytes(discriminant_bytes)
                };
                o = o + <$enum_discriminant_type>::BYTES_SIZE;
                match discriminant {
                    $(
                        $variant_discriminant => {
                            Self::$variant_identifier({
                                let mut payload = [0u8; <$variant_type>::BYTES_SIZE];
                                payload.copy_from_slice(&bytes[o..(o+<$variant_type>::BYTES_SIZE)]);
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
        }
    };
}
