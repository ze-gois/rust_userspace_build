#[macro_export]
macro_rules! define_error_nested{
    (
        $variant:ty,
        $label:expr,
        [
            $(
                [
                    $variant_identifier:ident,
                    $variant_path:ident,
                    $variant_constant:ident
                    $variant_discriminant:expr,
                    $variant_descriptor:expr,
                    $variant_acronym:expr,
                ]
            ),*
            $(,)?
        ]
    ) => {
        use $crate::ErrorNestedTrait;
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
            $($variant_identifier(_) = $variant_discriminant,)*
            TODO = <$variant>::MAX,
        }

        impl ErrorTrait<$variant> for Error {
            fn from_no(discriminant: $variant) -> Self {
                match discriminant {
                    $($variant_discriminant => Self::$variant_identifier::default(),)*
                    <$variant>::MAX => Self::TODO,
                }
            }

            fn to_no(&self) -> $variant {
                match *self {
                    $(Self::$variant_identifier(_) => $variant_discriminant,)*
                    _ => <$variant>::MAX,
                }
            }

            fn description(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier(_) => $variant_descriptor,)*
                    _ => "TODO",
                }
            }

            fn acronym(&self) -> &str {
                match *self {
                    $(Self::$variant_identifier(_) => $variant_acronym,)*
                    _ => "TODO",
                }
            }
        }

        $(
            use $variant_path::Result as variant_result;
            use $variant_path::Error as variant_error;
            use $variant_path::ERROR_LABEL as VARIANT_ERROR_LABEL;
            use $variant_path::ErrorType as variant_error_type;
        )*

        impl Into<$variant> for Error {
            fn into(self) -> $variant {
                self.to_no()
            }
        }

        pub type Result = core::result::Result<ErrorType, Error>;
    };
}
