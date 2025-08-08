use proc_macro::TokenStream;

mod result;

#[proc_macro]
pub fn define_error(input: TokenStream) -> TokenStream {
    result::error::expand_define_error(input)
}
