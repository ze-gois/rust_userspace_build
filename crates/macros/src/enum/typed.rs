/// A macro that defines an error type and error handling for syscalls.
///
/// This macro generates:
/// 1. An Error enum with the specified variants and their associated discriminant values
/// 2. ErrorTrait implementation for the Error type with proper discriminant mapping
/// 3. An discriminant module with standard Linux error constants
/// 4. Into<isize> implementation for the Error type
/// 5. A handle_result function that maps arch errors to syscall errors
///
/// # Arguments
///
/// * `$enum_identifier` - The name of the error enum (usually Error)
/// * `$variant` - The variant name in the crate::result::Error enum (e.g., Open, Read, Write)
/// * `$label` - String slice with the syscall name
/// * A list of error variants with their descriptions, discriminant values and Linux standard constant names
///   [VariantName, discriminant_value, "description", "LINUX_CONSTANT"]
///
/// # Example
///
/// ```
/// $enum_identifier:ident,
/// $variant:ty,
/// $label:expr,
/// [
///     $(
///         [
///             $discriminant:expr;
///             $identifier:ident;
///             $const_identifier:ident;
///             $acronym:expr;
///             $description:expr
///         ]
///     ),* $(,)?
/// ]
/// ```
#[macro_export]
#[rustfmt::skip]
macro_rules! enum_typed {
    (
        $enum_identifier:ident,
        $variant:ty,
        $label:expr,
        [$($($import:tt)::*);*],
        [
            $(
                [
                    $discriminant:expr;
                    $identifier:ident;
                    $type:ty;
                    |$argumento:ident : $tipo:ty|{$($lambda:tt)*};
                    $const_identifier:ident;
                    $acronym:expr;
                    $description:expr
                ]
            ),* $(,)?
        ]
    ) => {
        $(
            pub use $($import)::*;
        )*
        // Define Linux standard error constants in an discriminant module with standard names
        pub mod constants {
            $(
                pub const $const_identifier: $variant = $discriminant;
            )*
        }

        pub mod types {
            $(
                pub use $($import)::*;
            )*
            $(
                pub type $identifier = $type;
            )*
        }

        #[repr(C)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum $enum_identifier {
            $($identifier = $discriminant,)*
            TODO,
        }

        impl $enum_identifier {
            pub fn from(discriminant: $variant) -> Self {
                match discriminant {
                    $($discriminant => Self::$identifier,)*
                    _ => Self::TODO,
                }
            }

            pub fn to(&self) -> $variant {
                match *self {
                    $(Self::$identifier => $discriminant,)*
                    _ => <$variant>::MAX
                }
            }

            pub fn str(&self) -> &str {
                match self {
                    $(Self::$identifier => $description,)*
                    _ => "TODO"
                }
            }

            pub fn acronym(&self) -> &str {
                match *self {
                    $(Self::$identifier => $acronym,)*
                    _ => "Unknown error",
                }
            }
        }

        impl core::ops::Add for $enum_identifier {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $enum_identifier::from(self.to() | rhs.to())
            }
        }

        impl core::ops::Sub for $enum_identifier {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                $enum_identifier::from(self.to() & !rhs.to())
            }
        }

        impl core::fmt::Display for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}:{}",self.to(), self.str())
            }
        }

        impl core::fmt::Debug for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}:{}",self.to(), self.acronym())
            }
        }

        #[derive(Debug, Clone, Copy)]
        pub enum EnumTyped {
            $($identifier($type),)*
            TODO(u32),
        }

        impl EnumTyped {
            pub fn from_kv(etype: *const $variant, p: *const u8) -> Self {
                match $enum_identifier::from(unsafe { *etype }) {
                    $( $enum_identifier::$identifier => EnumTyped::$identifier( unsafe { (|$argumento:$tipo|{ $($lambda)* })(p)} ),)*
                    _ => EnumTyped::TODO(0),
                }
            }

            pub fn to_kv(&self) -> ($variant, *const u8) {
                match *self {
                    $(EnumTyped::$identifier(v) => (($enum_identifier::$identifier).to(), v as *const u8),)*
                    EnumTyped::TODO(id) => ($enum_identifier::TODO.to(), id as *const u8),
                }
            }

            pub fn is_null(&self) -> bool {
                match self {
                    EnumTyped::Null(0) => true,
                    _ => false,
                }
            }
        }
    };
}
