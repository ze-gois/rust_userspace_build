#[macro_export]
macro_rules! info {
    ($($t:tt)*) => {
        human::info!($($t)*);
    };
}
