#[macro_export]
macro_rules! define_error_nested{
    (
        $variant:ty,
        $label:expr,
        [
            $(
                [
                    $variant_identifier:ident,
                    $($variant_path:tt)+,
                    $variant_constant:ident,
                    $variant_discriminant:expr,
                    $variant_descriptor:expr,
                    $variant_acronym:expr
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

        define_error_nested!(@defined_erro_enum $variant; $(($($variant_path)+, $variant_identifier)),*);

        impl ErrorTrait<$variant> for Error {
            fn from_no(discriminant: $variant) -> Self {
                match discriminant {
                    $($variant_discriminant => Self::$variant_identifier::TODO,)*
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

        pub type Result = core::result::Result<ErrorType, Error>;

        $(
            use $variant_path::Result as variant_result;
            use $variant_path::Error as variant_error;
            use $variant_path::ERROR_LABEL as VARIANT_ERROR_LABEL;
            use $variant_path::ErrorType as variant_error_type;

            impl ErrorNestedType<ErrorType,variant_error_type> for Error {
                fn from_no(a:ErrorType, b:variant_error_type) {
                    // match (stringfy!())
                    // (

                    // )*
                }
            }
        )*

        impl Into<$variant> for Error {
            fn into(self) -> $variant {
                self.to_no()
            }
        }

    };

    (@define_error_enum_body $variant:ty; $(($variant_token:tt)),*) => {
        #[repr($variant)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Error {
            $(
                $variant_identifier
            )*
        }
    };

    (@variant_generator $(($variant_identifier:ident, $variant_path:tt)),*) => {
        $(
            define_error_nested!(@variant_flexor $variant_path:tt; $variant_identifier:ident),
        )*
        TODO = <$variant>::MAX,
    };

    (@variant_flexor self, $variant_identifier:ident) => {
        $variant_identifier
    };

    (@variant_flexor $variant_path:tt, $variant_identifier:ident) => {
        $variant_identifier(<$variant_path>::Error)
    };

    (@defined_error_enum $variant:ty; $($variant_path:tt, $variant_identifier:ident),*) => {
        define_error_nested!(
            @define_error_enum_body
            $variant;
            define_error_nested!(@variant_generator $(($variant_identifier, $variant_path)),*);
        );
    };
}
