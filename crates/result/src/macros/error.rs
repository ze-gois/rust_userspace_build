#[macro_export]
macro_rules! define_error{
    (
        $variant:ty,
        $label:expr,
        [
            $(
                [
                    $variant_identifier:ident,
                    $variant_discriminant:expr,
                    $variant_descriptor:expr,
                    $variant_acronym:expr,
                    $variant_constant:ident
                ]
            ),*
            $(,)?
        ]
    ) => {
        use $crate::ErrorTrait;

        pub type ErrorType = $variant;
        pub const ERROR_LABEL : &str = $label;

        pub mod constant {
            $(
                const $variant_constant : $variant = $variant_discriminant;
            )*
        }

        #[repr($variant)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Error {
            $($variant_identifier = $variant_discriminant,)*
            TODO,
        }

        impl ErrorTrait<$variant> for Error {
            fn from_no(discriminant: $variant) -> Self {
                match discriminant {
                    $($variant_discriminant => Self::$variant_identifier,)*
                    _ => Self::TODO,
                }
            }

            fn to_no(&self) -> $variant {
                match *self {
                    $(Self::$variant_identifier => $variant_discriminant,)*
                    _ => <$variant>::MAX,
                }
            }

            fn description(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier => $variant_descriptor,)*
                    _ => "TODO",
                }
            }

            fn acronym(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier => $variant_acronym,)*
                    _ => "TODO",
                }
            }
        }

        impl Into<$variant> for Error {
            fn into(self) -> $variant {
                self.to_no()
            }
        }

        impl Default for Error {
            fn default() -> Self {
                Self::TODO
            }
        }
        pub type Result = core::result::Result<ErrorType, Error>;
    };
}
