#[macro_export]
#[rustfmt::skip]
macro_rules! default_error {
    (
        $label:expr;
        $($module:tt)::*;
        [
            $(
                [
                    $discriminant:expr;
                    $identifier:ident;
                    $type:ty;
                    $const_identifier:ident;
                    $acronym:expr;
                    $description:expr
                ]
            ),* $(,)?
        ]
    ) => {

    }
}
