#[macro_export]
macro_rules! r#struct {
    ($vis:vis $struct_identifier:ident {$($field_identifier:ident: $field_type:ty),*$(,)?}) => {

        // #[repr(C,packed)]
        #[derive(Debug, Clone, Copy)]
        $vis struct $struct_identifier {
            $($field_identifier: $field_type),*
        }

        impl BytesTrait for $struct_identifier {
            const BYTES_SIZE : usize = $(<$field_type>::BYTES_SIZE +)* 0;

            fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
                let mut b = [0u8; Self::BYTES_SIZE];
                let mut o = 0;
                $(
                    b[o..(o+<$field_type>::BYTES_SIZE)].copy_from_slice(
                        &if endianness {
                            self.$field_identifier.to_le_bytes()
                        } else {
                            self.$field_identifier.to_be_bytes()
                        }
                    );
                    o = o + <$field_type>::BYTES_SIZE;
                )*
                b
            }

            fn from_bytes(bytes : [u8; Self::BYTES_SIZE], endianness: bool) -> Self {
                let mut o = 0;
                $(
                    let mut field_bytes = [0u8; <$field_type>::BYTES_SIZE];
                    field_bytes.copy_from_slice(&bytes[o..(o+<$field_type>::BYTES_SIZE)]);
                    let $field_identifier = if endianness {
                        <$field_type>::from_le_bytes(field_bytes)
                    } else {
                        <$field_type>::from_be_bytes(field_bytes)
                    };
                    o = o + <$field_type>::BYTES_SIZE;
                )*
                Self {
                    $($field_identifier,)*
                }
            }
        }
    }
}
