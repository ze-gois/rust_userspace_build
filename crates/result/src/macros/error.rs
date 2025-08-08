#[macro_export]
macro_rules! define_error{
    (
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

        pub const LABEL : &str = $label;

        pub mod constant {
            $(
                const $variant_constant : usize = $variant_discriminant;
            )*
        }

        #[repr(usize)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Error {
            $($variant_identifier = $variant_discriminant,)*
            TODO = <usize>::MAX,
        }

        impl ErrorTrait for Error {
            fn from_no(discriminant: usize) -> Self {
                match discriminant {
                    $($variant_discriminant => Self::$variant_identifier,)*
                    _ => Self::TODO,
                }
            }

            fn to_no(&self) -> usize {
                match *self {
                    $(Self::$variant_identifier => $variant_discriminant,)*
                    _ => <usize>::MAX,
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

        impl Into<usize> for Error {
            fn into(self) -> usize {
                self.to_no()
            }
        }

        impl Into<isize> for Error {
            fn into(self) -> isize {
                self.to_no() as isize
            }
        }

        impl Default for Error {
            fn default() -> Self {
                Self::TODO
            }
        }
        pub type Result = core::result::Result<usize, Error>;
    };
}
