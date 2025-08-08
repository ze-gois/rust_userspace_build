pub struct Variant {
    pub identifier: syn::Ident,
    pub discriminant: syn::Expr,
    pub description: syn::LitStr,
    pub acronym: syn::LitStr,
    pub constant: syn::Ident,
}

pub struct Error {
    pub discriminant: syn::Type,
    pub label: syn::LitStr,
    pub variants: Vec<Variant>,
}
