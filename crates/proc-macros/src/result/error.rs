mod generate;
mod model;
mod parser;

pub fn expand_define_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as model::Error);
    generate::generate_define_error(input).into()
}
