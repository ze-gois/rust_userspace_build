#[macro_export]
macro_rules! define_error_nested {
    (
        $label:expr,
        [
            $(
                [
                    $variant_identifier:ident;
                    $($variant_path:tt)::+;
                    $variant_constant:ident,
                    $variant_discriminant:expr,
                    $variant_descriptor:expr,
                    $variant_acronym:expr
                ]
            ),*
        ]
    ) => {
        // use $crate::ErrorNestedTrait;
        // use $crate::ErrorTrait;

        pub const LABEL : &str = $label;

        pub mod constant {
            $(
                const $variant_constant : usize = $variant_discriminant;
            )*
        }

        #[repr(usize)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Error {
            $(
                $variant_identifier($($variant_path)::+::Error),
            )+
            TODO
        }

        impl $crate::ErrorTrait for Error {
            fn from_no(discriminant: usize) -> Error {
                match discriminant {
                    $($variant_discriminant => Error::$variant_identifier($($variant_path)::+::Error::TODO),)*
                    _ => Self::TODO,
                }
            }

            fn to_no(&self) -> usize {
                match *self {
                    $(Error::$variant_identifier(_) => $variant_discriminant,)*
                    _ => <usize>::MAX,
                }
            }

            fn description(&self) -> &str {
                match *self {
                    $(Error::$variant_identifier(_) => $variant_descriptor,)*
                    _ => "TODO",
                }
            }

            fn acronym(&self) -> &str {
                match *self {
                    $(Error::$variant_identifier(_) => $variant_acronym,)*
                    _ => "TODO",
                }
            }
        }

        impl $crate::ErrorNestedTrait for Error {
            fn from_no(a:usize, b:usize) -> Error{
                use $crate::ErrorTrait;
                match <Error as ErrorTrait>::from_no(a) {
                    $(
                        Error::$variant_identifier($($variant_path)::+::Error::TODO) => Error::$variant_identifier($($variant_path)::+::Error::from_no(b)),
                    )+
                    _ => Error::TODO
                }
            }
            fn to_no(&self) -> (usize, usize) {
                use $crate::ErrorTrait;
                match self {
                    $(
                        Error::$variant_identifier(variant) => {
                            ($crate::ErrorTrait::to_no(self), variant.to_no())
                        }
                    ),*
                    _ => (<usize>::MAX, <usize>::MAX)
                }
            }

            fn description(&self) -> &str {
                match self {
                    $(
                        Error::$variant_identifier(variant) => {
                            // concat!(self.description(), "(", variant.description(), ")")
                            concat!($label, "(", $variant_descriptor, ")")
                        }
                    ),*
                    _ => "TODO"
                }
            }

            fn acronym(&self) -> &str {
                match self {
                    $(
                        Error::$variant_identifier(variant) => {
                            // concat!(self.acronym(), "(", variant.acronym(), ")")
                            concat!($label, "(", $variant_acronym, ")")
                        }
                    ),*
                    _ => "TODO"
                }
            }
        }

        impl Into<usize> for Error {
            fn into(self) -> usize {
                use $crate::ErrorTrait;
                self.to_no()
            }
        }

        impl Into<(usize,usize)> for Error {
            fn into(self) -> (usize,usize) {
                use $crate::ErrorTrait;
                match self {
                    $(
                        Error::$variant_identifier(variant) => (self.to_no(), variant.to_no()),
                    )*
                    _ => (<usize>::MAX,<usize>::MAX)
                }
            }
        }

        impl From<usize> for Error {
            fn from(a:usize) -> Error {
                use result::ErrorTrait;
                Error::from_no(a)
            }
        }

        impl From<(usize,usize)> for Error {
            fn from(a: (usize,usize)) -> Error {
                use $crate::ErrorNestedTrait;
                Error::from_no(a.0, a.1)
            }
        }

        pub type Result = core::result::Result<(usize,usize), Error>;
    };
}
