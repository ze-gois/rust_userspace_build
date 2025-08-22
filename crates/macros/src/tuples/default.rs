#[macro_export]
macro_rules! r#tuple {
    ($struct_visualization:vis $struct_identifier:ident, $($field_type:ty),*) => {
        $struct_visualization struct $struct_identifier($($field_type),*);

        pub use ::macros::traits::XElfSize;

        impl XElfSize for $struct_identifier {
            const SIZE: usize = $(<$field_type>::XELF_SIZE + )* 0;
        }


        impl $struct_identifier {
            pub fn to_bytes(&self, endianness: bool) -> [u8; $struct_identifier::XELF_SIZE] {
                let mut b = [0u8; $struct_identifier::XELF_SIZE];
                let mut o = 0;
                $(
                    b[o..(o+<$field_type>::XELF_SIZE)].copy_from_slice(
                        &if endianness {
                            self.$field_identifier.to_le_bytes()
                        } else {
                            self.$field_identifier.to_be_bytes()
                        }
                    );
                    o = o + <$field_type>::XELF_SIZE;
                )*
                b
            }

            pub fn to_le_bytes(&self) -> [u8; $struct_identifier::XELF_SIZE] {
                self.to_bytes(true)
            }

            pub fn to_be_bytes(&self) -> [u8; $struct_identifier::XELF_SIZE] {
                self.to_bytes(false)
            }

            pub fn from_bytes(bytes : [u8; $struct_identifier::XELF_SIZE], endianness: bool) -> $struct_identifier {
                let mut o = 0;
                $(
                    let mut field_bytes = [0u8; <$field_type>::XELF_SIZE];
                    field_bytes.copy_from_slice(&bytes[o..(o+<$field_type>::XELF_SIZE)]);
                    let $field_identifier = if endianness {
                        <$field_type>::from_le_bytes(field_bytes)
                    } else {
                        <$field_type>::from_be_bytes(field_bytes)
                    };
                    o = o + <$field_type>::XELF_SIZE;
                )*
                $struct_identifier {
                    $($field_identifier,)*
                }
            }

            pub fn from_le_bytes(bytes : [u8; $struct_identifier::XELF_SIZE]) -> $struct_identifier {
                $struct_identifier::from_bytes(bytes,true)
            }

            pub fn from_be_bytes(bytes : [u8; $struct_identifier::XELF_SIZE]) -> $struct_identifier {
                $struct_identifier::from_bytes(bytes,false)
            }
        }

    };
}
