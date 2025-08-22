#[macro_export]
macro_rules! trait_implement_primitive {
    ($tv:expr, $($t:ty),*) => {
        $(
            impl $crate::macros::traits::Primitive for $t {
                const IS_PRIMITIVE: bool = $tv;
            }
        )*
    };
}
