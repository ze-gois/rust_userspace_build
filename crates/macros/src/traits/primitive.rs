pub trait Primitive<Observer, Reference> {
    const IS_PRIMITIVE: bool;
}

#[macro_export]
macro_rules! trait_implement_primitive {
    ($tv:expr, $($t:ty),*) => {
        $(
            impl macros::traits::Primitive<crate::Origin,crate::Origin> for $t {
                const IS_PRIMITIVE: bool = $tv;
            }
        )*
    };
}

// #[macro_export]
// macro_rules! trait_place_primitive {
//     () => {
//         pub trait Primitive {
//             const IS_PRIMITIVE: bool;
//         }
//     };
// }
