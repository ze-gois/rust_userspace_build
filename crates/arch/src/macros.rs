#[macro_export]
macro_rules! wrap_syscall {
    ($name:ident, $syscall:ident, $($arg:ident : $type:ty),*) => {
        fn $name($($arg: $type,)*) -> $crate::Result {
            let return_value = $crate::Arch::$syscall($($arg),*);
            return_value
        }
    }
}
