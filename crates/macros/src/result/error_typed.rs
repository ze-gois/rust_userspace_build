#[macro_export]
#[rustfmt::skip]
macro_rules! error_typed {
    (
        $label:expr;
        $($module:tt)::*;
        [
            $(
                [
                    $discriminant:expr;
                    $identifier:ident;
                    $type:ty;
                    $argumento:ident;
                    {$($lambda_from_franco:tt)*};
                    {$($lambda_to_franco:tt)*};
                    $const_identifier:ident;
                    $acronym:expr;
                    $description:expr
                ]
            ),* $(,)?
        ]
    ) => {
        pub type Franco = *const u8;

        pub use $($module)::*::*;

        // Define Linux standard error constants in an discriminant module with standard names
        pub mod constants {
            $(
                pub const $const_identifier: usize = $discriminant;
            )*
        }

        pub mod types {
            pub use $($module)::*::*;
            $(
                pub type $identifier = $type;
            )*
        }

        #[derive(Debug, Clone, Copy)]
        pub enum Error {
            $($identifier($type),)*
            TODO,
        }

        impl Error {
            pub fn discriminant(&self) -> usize {
                match *self {
                    $( Error::$identifier(_) => $discriminant,)*
                    Error::TODO => usize::MAX,
                }
            }

            pub fn from_dp(discriminant: usize, value_pointer: Franco) -> Self {
                match discriminant {
                    $( $discriminant => Error::$identifier(unsafe { (|$argumento:Franco|{ $($lambda_from_franco)* })(value_pointer) }),)*
                    _ => Error::TODO,
                }
            }

            pub fn to_dp(&self) -> (usize, Franco) {
                match *self {
                    $( Error::$identifier(value) => (self.discriminant(), (|$argumento:$type|{ $($lambda_to_franco)* })(value)),)*
                    Error::TODO => (Error::TODO.discriminant(), core::ptr::null_mut()),
                }
            }
        }
    };
}
