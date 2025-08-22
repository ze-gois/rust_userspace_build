#[macro_export]
macro_rules! trait_implement_xelf_size {
    ($($t:ty),*) => {
        $(
            impl crate::macros::traits::XElfSize for $t {
                const XELF_SIZE : usize = core::mem::size_of::<$t>();
            }
        )*
    };
}
