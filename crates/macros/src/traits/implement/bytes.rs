#[macro_export]
macro_rules! trait_implement_bytes {
    ($($t:ty),*) => {
        $(
            impl crate::macros::traits::Bytes for $t {
                const BYTES_SIZE: usize = core::mem::size_of::<Self>();

                fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
                    if endianness {
                        self.to_le_bytes()
                    } else {
                        self.to_be_bytes()
                    }
                }

                fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self {
                    if endianness {
                        Self::from_le_bytes(bytes)
                    } else {
                        Self::from_be_bytes(bytes)
                    }
                }
            }
        )*
    };
}
