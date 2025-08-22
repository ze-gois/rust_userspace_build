#[macro_export]
macro_rules! expressions_lowerbound {
    // caso base: apenas um argumento
    ($x:expr) => { $x };
    // caso recursivo: pelo menos dois
    ($x:expr, $($rest:expr),+) => {
        {
            let y = ::macros::expressions_lowerbound!($($rest),+);
            if $x > y { y } else { $x }
        }
    };
}
