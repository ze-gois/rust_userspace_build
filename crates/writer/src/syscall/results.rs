// pub use crate::results::traits::Result as ResultTrait;

pub mod ok {
    results::result!(
        Ok;
        "x86_64 ok";
        usize;
        [
            [1; UNOTICED; UnoticedWrite; usize; "Unoticed"; "We didn't evaluate it"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::UnoticedWrite(no)
        }
    }
}

pub mod error {
    results::result!(
        Error;
        "x86_64 error";
        usize;
        [
            [1; UNOTICED; UnoticedWriteErr; usize; "Unoticed"; "We didn't evaluate it"],
        ]
    );

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::UnoticedWriteErr(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

// crate::macros::r#enum!(pub Result, u8, [
//     [0, Ok, Ok],
//     [1, Err, Error]
// ]);

// pub type Result = ;
// use crate::macros::traits::Bytes as BytesTrait;

// impl<T, E> BytesTrait for core::result::Result<T, E>
// where
//     T: BytesTrait,
//     E: BytesTrait,
// {
//     const BYTES_SIZE: usize =
//         u8::BYTES_SIZE + crate::macros::expressions_upperbound!(T::BYTES_SIZE, E::BYTES_SIZE);

//     fn to_bytes(&self, endianness: bool) -> [u8; Self::BYTES_SIZE] {
//         let mut bytes = [0u8; Self::BYTES_SIZE];
//         if endianness {
//             match self {
//                 Self::Ok(o) => {
//                     bytes[0..u8::BYTES_SIZE].copy_from_slice(&0u8.to_le_bytes());
//                     bytes[u8::BYTES_SIZE..].copy_from_slice(&o.to_le_bytes());
//                 }
//                 Self::Err(e) => {
//                     bytes[0..u8::BYTES_SIZE].copy_from_slice(&1u8.to_le_bytes());
//                     bytes[u8::BYTES_SIZE..].copy_from_slice(&e.to_le_bytes());
//                 }
//             };
//         } else {
//             match self {
//                 Self::Ok(o) => {
//                     bytes[0..u8::BYTES_SIZE].copy_from_slice(&0u8.to_be_bytes());
//                     bytes[u8::BYTES_SIZE..].copy_from_slice(&o.to_be_bytes());
//                 }
//                 Self::Err(e) => {
//                     bytes[0..u8::BYTES_SIZE].copy_from_slice(&0u8.to_be_bytes());
//                     bytes[u8::BYTES_SIZE..].copy_from_slice(&e.to_be_bytes());
//                 }
//             }
//         };
//         bytes
//     }

//     fn from_bytes(bytes: [u8; Self::BYTES_SIZE], endianness: bool) -> Self
//     where
//         [u8; Self::BYTES_SIZE]:,
//     {
//         let mut o = 0;
//         let mut discriminant_bytes = [0u8; <u8 as BytesTrait>::BYTES_SIZE];
//         discriminant_bytes.copy_from_slice(&bytes[o..(o + <u8 as BytesTrait>::BYTES_SIZE)]);
//         let discriminant = if endianness {
//             <u8 as BytesTrait>::from_le_bytes(discriminant_bytes)
//         } else {
//             <u8 as BytesTrait>::from_be_bytes(discriminant_bytes)
//         };
//         o = o + <u8 as BytesTrait>::BYTES_SIZE;

//         match discriminant {
//             0u8 => {
//                 let mut payload_bytes = [0u8; T::BYTES_SIZE];
//                 payload_bytes.copy_from_slice(&bytes[o..(o + <T as BytesTrait>::BYTES_SIZE)]);
//                 let payload = if endianness {
//                     T::from_le_bytes(payload_bytes)
//                 } else {
//                     T::from_be_bytes(payload_bytes)
//                 };
//                 core::result::Result::Ok(payload)
//             }
//             1u8 => {
//                 let mut payload = [0u8; E::BYTES_SIZE];
//                 payload.copy_from_slice(&bytes[o..(o + <E as BytesTrait>::BYTES_SIZE)]);
//                 let payload = if endianness {
//                     <E as BytesTrait>::from_le_bytes(payload)
//                 } else {
//                     <E as BytesTrait>::from_be_bytes(payload)
//                 };
//                 core::result::Result::Err(payload)
//             }
//             _ => unreachable!(),
//         }
//     }
// }

pub type Result = core::result::Result<Ok, Error>;

// pub fn handle_result(result: usize) -> crate::Result {
//     if (result as isize) < 0 {
//         Err(crate::Error::Syscall(crate::results::error::SyscallEntry{}))
//     } else {
//         Ok(Ok::from_no(result))
//     }
// }
