// pub mod traits {
//     macros::trait_place_bytes!(1);
//     macros::trait_place_primitive!();
// }

// macros::trait_implement_defaut_for_primitives!();

// pub use macros::r#enum;
// pub use macros::expressions_upperbound;
// pub use macros::r#struct;
// pub use macros::trait_implement_bytes;
// pub use macros::trait_implement_defaut_for_primitives;
// pub use macros::trait_implement_primitive;

#[macro_export]
macro_rules! wrap_syscall {
    ($name:ident, $syscall:ident, $($arg:ident : $type:ty),*) => {
        fn $name($($arg: $type,)*) -> $crate::Result {
            let return_value = $crate::Arch::$syscall($($arg),*);
            $crate::info!(X "\tWrapped syscall");
            $($crate::info!(X "\t{:?},",$arg);)*
            $crate::info!(X "\t{:?}",return_value);

            return_value
        }
    }
}
