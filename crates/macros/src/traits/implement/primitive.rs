#[macro_export]
macro_rules! trait_implement_primitive {
    ($tv:expr, $($t:ty),*) => {
        $(
            impl crate::macros::traits::Primitive for $t {
                const IS_PRIMITIVE: bool = $tv;
            }
        )*
    };

    ($crate_id:ident,$tv:expr, $($t:ty),*) => {
        impl<T:$crate_id> crate::macros::traits::Primitive for T {
            const IS_PRIMITIVE: bool = $tv;
        }
        $(
            impl $crate_id for $t {}
        )*
    };
}
