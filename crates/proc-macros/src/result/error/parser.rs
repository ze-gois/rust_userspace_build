use super::model;

use syn::{
    Expr, Ident, LitStr, Result, Token, Type, braced,
    parse::{Parse, ParseStream},
};

impl Parse for model::Error {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Ident>()?; // discriminant:
        input.parse::<Token![:]>()?;
        let discriminant: Type = input.parse()?;
        input.parse::<Token![,]>()?;

        input.parse::<Ident>()?; // label:
        input.parse::<Token![:]>()?;
        let label: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        input.parse::<Ident>()?; // variants:
        input.parse::<Token![:]>()?;
        let content;
        syn::bracketed!(content in input);

        let mut variants = Vec::new();
        while !content.is_empty() {
            let inner;
            braced!(inner in content);

            let identifier: Ident = inner.parse()?;
            inner.parse::<Token![,]>()?;
            let discriminant: Expr = inner.parse()?;
            inner.parse::<Token![,]>()?;
            let description: LitStr = inner.parse()?;
            inner.parse::<Token![,]>()?;
            let acronym: LitStr = inner.parse()?;
            inner.parse::<Token![,]>()?;
            let constant: Ident = inner.parse()?;

            variants.push(model::Variant {
                identifier,
                discriminant,
                description,
                acronym,
                constant,
            });

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(model::Error {
            discriminant,
            label,
            variants,
        })
    }
}

// impl syn::parse::Parse for model::Error {
//     fn parse(input: syn::parse::ParseStream) -> Result<Self> {
//         input.parse::<syn::Ident>()?; // discriminant:
//         input.parse::<Token![:]>()?;
//         let discriminant: syn::Type = input.parse()?;
//         input.parse::<Token![,]>()?;

//         input.parse::<syn::Ident>()?; // label:
//         input.parse::<Token![:]>()?;
//         let label: LitStr = input.parse()?;
//         input.parse::<Token![,]>()?;

//         input.parse::<syn::Ident>()?; // variants:
//         input.parse::<Token![:]>()?;
//         let content;
//         syn::bracketed!(content in input);

//         let mut variants = Vec::new();
//         while !content.is_empty() {
//             let content_inner;
//             syn::braced!(content_inner in content);

//             let ident: Ident = content_inner.parse()?;
//             content_inner.parse::<Token![:]>()?;
//             content_inner.parse::<Token![=]>()?;
//             content_inner.parse::<Token![(]>()?;
//             let number: syn::Expr = content_inner.parse()?;
//             content_inner.parse::<Token![,]>()?;
//             let description: LitStr = content_inner.parse()?;
//             content_inner.parse::<Token![,]>()?;
//             let acronym: LitStr = content_inner.parse()?;
//             content_inner.parse::<Token![,]>()?;
//             let constant: Ident = content_inner.parse()?;
//             content_inner.parse::<Token![)]>()?;

//             variants.push(model::Variant {
//                 ident,
//                 number,
//                 description,
//                 acronym,
//                 constant,
//             });

//             if content.peek(Token![,]) {
//                 content.parse::<Token![,]>()?;
//             }
//         }

//         Ok(model::Error {
//             discriminant,
//             label,
//             variants,
//         })
//     }
// }
