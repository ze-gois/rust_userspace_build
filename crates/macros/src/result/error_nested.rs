#[macro_export]
#[rustfmt::skip]
/// $label:expr,
/// [
///     $(
///         [
///             $variant_discriminant:expr;
///             $variant_identifier:ident;
///             $($variant_path:tt)::+;
///             $variant_constant:ident;
///             $variant_descriptor:expr;
///             $variant_acronym:expr
///         ]
///     ),*
///     $(,)*
/// ]
///
macro_rules! define_error_nested {
    (
        $label:expr,
        [
            $(
                [
                    $variant_discriminant:expr;
                    $variant_identifier:ident;
                    $($variant_path:tt)::+;
                    $variant_constant:ident;
                    $variant_descriptor:expr;
                    $variant_acronym:expr
                ]
            ),*
            $(,)*
        ]
    ) => {

        pub mod constant {
            pub const LABEL : &str = $label;

            $(
                const $variant_constant : usize = $variant_discriminant;
            )*
        }

        #[repr(usize)]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Error {
            $(
                $variant_identifier($($variant_path)::+::Error),
            )+
            TODO
        }

        impl ::macros::result::ErrorTrait for Error {
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

            fn from_ptr(ptr: *const u8) -> Error {
                Self::from_no(unsafe{*(ptr as *const usize)})
            }
        }

        impl ::macros::result::ErrorNestedTrait for Error {
            fn from_no(a:usize, b:usize) -> Error{
                use ::macros::result::ErrorTrait;
                match <Error as ErrorTrait>::from_no(a) {
                    $(
                        Error::$variant_identifier(_) =>{
                            let variant = <$($variant_path)::+::Error as ErrorTrait>::from_no(b);
                            Error::$variant_identifier(variant)
                        }
                    )+
                    _ => Error::TODO
                }
            }
            fn to_no(&self) -> (usize, usize) {
                use ::macros::result::ErrorTrait;
                match self {
                    $(
                        Error::$variant_identifier(variant) => {
                            (ErrorTrait::to_no(self), ErrorTrait::to_no(variant))
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
                use ::macros::result::ErrorTrait;
                self.to_no()
            }
        }

        impl Into<(usize,usize)> for Error {
            fn into(self) -> (usize,usize) {
                use ::macros::result::ErrorTrait;
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
                use ::macros::result::ErrorTrait;
                Error::from_no(a)
            }
        }

        impl From<(usize,usize)> for Error {
            fn from(a: (usize,usize)) -> Error {
                use ::macros::result::ErrorNestedTrait;
                Error::from_no(a.0, a.1)
            }
        }

        pub type Result = core::result::Result<(usize,usize), Error>;
    };
}
