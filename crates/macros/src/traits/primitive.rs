#[macro_export]
macro_rules! trait_place_primitive {
    () => {
        pub trait Primitive {
            const IS_PRIMITIVE: bool;
        }
    };
}
