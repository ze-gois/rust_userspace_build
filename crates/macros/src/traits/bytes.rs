pub trait Bytes<Observer, Reference> {
    const BYTES_SIZE: usize;

    fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE];
    fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE] {
        self.to_bytes(true)
    }
    fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE] {
        self.to_bytes(false)
    }

    fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self;
    fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
    where
        Self: Sized,
    {
        Self::from_bytes(bytes, true)
    }
    fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
    where
        Self: Sized,
    {
        Self::from_bytes(bytes, false)
    }
}

// pub trait Bytes<Observer, Reference> {
//     const BYTES_SIZE: usize;

//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE]
//     where
//         [u8; Self::BYTES_SIZE]:;
//     fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE]
//     where
//         [u8; Self::BYTES_SIZE]:,
//     {
//         self.to_bytes(true)
//     }
//     fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE]
//     where
//         [u8; Self::BYTES_SIZE]:,
//     {
//         self.to_bytes(false)
//     }
//     fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self
//     where
//         [u8; Self::BYTES_SIZE]:;
//     fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//         [u8; Self::BYTES_SIZE]:,
//     {
//         Self::from_bytes(bytes, true)
//     }
//     fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//         [u8; Self::BYTES_SIZE]:,
//     {
//         Self::from_bytes(bytes, false)
//     }
// }

// pub trait Bytes<Observer, Reference> {
//     const BYTES_SIZE: usize;

//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE];
//     fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE] {
//         self.to_bytes(true)
//     }
//     fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE] {
//         self.to_bytes(false)
//     }
//     fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self;
//     fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//     {
//         Self::from_bytes(bytes, true)
//     }
//     fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//     where
//         Self: Sized,
//     {
//         Self::from_bytes(bytes, false)
//     }
// }

#[macro_export]
macro_rules! trait_implement_bytes {
    ($($t:ty),*) => {
        $(
            impl macros::traits::Bytes<crate::Origin,crate::Origin> for $t {
                const BYTES_SIZE: usize = core::mem::size_of::<Self>();

                fn to_bytes(&self, endianness: bool) -> [u8; <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE] {
                    if endianness {
                        <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::to_le_bytes(self)
                    } else {
                        <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::to_be_bytes(self)
                    }
                }

                fn from_bytes(bytes: [u8; <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                    if endianness {
                        <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(bytes)
                    } else {
                        <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(bytes)
                    }
                }
            }

            impl macros::traits::Bytes<crate::Origin,crate::Origin> for Option<$t> {
                const BYTES_SIZE: usize = core::mem::size_of::<u8>() + <$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                fn from_bytes(bytes: [u8; <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE], endianness: bool) -> Self {
                    let mut option_bytes = [0u8; <u8 as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                    let mut o = 0;
                    let mut l = <u8 as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                    option_bytes.copy_from_slice(&bytes[o..l]);
                    let option = if endianness {
                        <u8 as macros::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(option_bytes)
                    } else {
                        <u8 as macros::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(option_bytes)
                    };
                    if option == 0 {
                        None
                    } else {
                        o = l;
                        l = l + <$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                        let mut value_bytes = [0u8; <$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                        value_bytes.copy_from_slice(&bytes[o..l]);
                        if endianness {
                            Some(<$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::from_le_bytes(value_bytes))
                        } else {
                            Some(<$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::from_be_bytes(value_bytes))
                        }
                    }
                }

                fn to_bytes(&self, endianness: bool) -> [u8; <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE] {
                    let mut bytes = [0u8; <Self as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE];
                    if let Some(v) = self {
                        let mut o = 0;
                        let mut l = <u8 as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                        bytes[o..l].copy_from_slice(&1u8.to_le_bytes());
                        o = l;
                        l = l + <$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::BYTES_SIZE;
                        if endianness {
                            bytes[o..l].copy_from_slice(&<$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::to_le_bytes(v));
                        } else {
                            bytes[o..l].copy_from_slice(&<$t as macros::traits::Bytes<crate::Origin,crate::Origin>>::to_be_bytes(v));
                        }
                        bytes
                    } else {
                        bytes
                    }
                }
            }
        )*
    };
}

// #[macro_export]
// macro_rules! trait_place_bytes {
//     () => {
//         pub trait Bytes {
//             const BYTES_SIZE: usize;

//             fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE];
//             fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE] {
//                 self.to_bytes(true)
//             }
//             fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE] {
//                 self.to_bytes(false)
//             }
//             fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self;
//             fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//             where
//                 Self: Sized,
//             {
//                 Self::from_bytes(bytes, true)
//             }
//             fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//             where
//                 Self: Sized,
//             {
//                 Self::from_bytes(bytes, false)
//             }
//         }
//     };
//     (1) => {
//         pub trait Bytes {
//             const BYTES_SIZE: usize;

//             fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE]
//             where
//                 [u8; Self::BYTES_SIZE]:;
//             fn to_le_bytes(&self) -> [u8; Self::BYTES_SIZE]
//             where
//                 [u8; Self::BYTES_SIZE]:,
//             {
//                 self.to_bytes(true)
//             }
//             fn to_be_bytes(&self) -> [u8; Self::BYTES_SIZE]
//             where
//                 [u8; Self::BYTES_SIZE]:,
//             {
//                 self.to_bytes(false)
//             }
//             fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self
//             where
//                 [u8; Self::BYTES_SIZE]:;
//             fn from_le_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//             where
//                 Self: Sized,
//                 [u8; Self::BYTES_SIZE]:,
//             {
//                 Self::from_bytes(bytes, true)
//             }
//             fn from_be_bytes(bytes: [u8; Self::BYTES_SIZE]) -> Self
//             where
//                 Self: Sized,
//                 [u8; Self::BYTES_SIZE]:,
//             {
//                 Self::from_bytes(bytes, false)
//             }
//         }
//     };
// }
