#[macro_export]
macro_rules! labeled_enum_typed {
    ($enum_identifier:ident, $variant:ty, $content:ty, $label:expr,
     [ $( [$identifier:ident, $const_identifier:ident, $discriminant:expr, $description:expr, $acronym:expr] ),* $(,)? ]) => {
        // Define Linux standard error constants in an discriminant module with standard names
        pub mod constants {
            $(
                pub const $const_identifier: $variant = $discriminant;
            )*
        }

        #[repr(C)]
        #[repr($variant)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum $enum_identifier {
            $($identifier($content) = $discriminant,)*
            TODO,
        }

        impl $enum_identifier {
            pub fn from(discriminant: $variant) -> Self {
                match discriminant {
                    $($discriminant => Self::$identifier(<$content>::default()),)*
                    _ => Self::TODO,
                }
            }

            pub fn to(&self) -> $variant {
                match *self {
                    $(Self::$identifier(_) => $discriminant,)*
                    _ => <$variant>::MAX
                }
            }

            pub fn str(&self) -> &str {
                match self {
                    $(Self::$identifier(_) => $description,)*
                    _ => "TODO"
                }
            }

            pub fn acronym(&self) -> &str {
                match *self {
                    $(Self::$identifier(_) => $acronym,)*
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
                write!(f, "{}", self.str())
            }
        }

        impl core::fmt::Debug for $enum_identifier {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.acronym())
            }
        }
    };
}
