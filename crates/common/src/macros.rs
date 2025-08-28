macros::trait_implement_primitives!();

#[macro_export]
macro_rules! info {
    ($($token:tt)*) => {
        target::info!($($token)*)
    };
}
