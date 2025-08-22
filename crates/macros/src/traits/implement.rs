pub mod from;

pub mod partial_eq;

pub mod primitive;

pub mod xelf_size;

#[macro_export]
macro_rules! trait_implement_default_primitive {
    ($($t:ty),*) => {
        trait_implement_xelf_size!($($t),*);
        trait_implement_primitive!($($t),*);
    };
}
