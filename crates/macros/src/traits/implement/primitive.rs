#[macro_export]
macro_rules! trait_implement_primitive {
    ($($t:ty),*) => {
       $(
           impl crate::macros::traits::Primitive for $t {
               const IS_PRIMITIVE: bool = true;
           }
       )*
    };
}
