use quote::quote;

use super::model::Error;

pub fn generate_define_error(error: Error) -> proc_macro2::TokenStream {
    let error_discriminant = error.discriminant;
    let label = error.label;
    let variants = error.variants;

    let error_variants = variants.iter().map(|v| {
        let name = &v.identifier;
        let discriminant = &v.discriminant;
        quote! { #name = #discriminant, }
    });

    let from_no_matches = variants.iter().map(|v| {
        let name = &v.identifier;
        let discriminant = &v.discriminant;
        quote! { #discriminant => Self::#name, }
    });

    let to_no_matches = variants.iter().map(|v| {
        let name = &v.identifier;
        let discriminant = &v.discriminant;
        quote! { Self::#name => #discriminant, }
    });

    let desc_matches = variants.iter().map(|v| {
        let name = &v.identifier;
        let desc = &v.description;
        quote! { Self::#name => #desc, }
    });

    let acr_matches = variants.iter().map(|v| {
        let name = &v.identifier;
        let acr = &v.acronym;
        quote! { Self::#name => #acr, }
    });

    let consts = variants.iter().map(|v| {
        let name = &v.constant;
        let discriminant = &v.discriminant;
        quote! { const #name: #error_discriminant = #discriminant; }
    });

    quote! {
        pub type ErrorType = #error_discriminant;
        pub const ERROR_LABEL: &str = #label;

        pub mod constant {
            #(#consts)*
        }

        #[repr(#error_discriminant)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub enum Error {
            #(#error_variants)*
            TODO,
        }

        impl $crate::ErrorTrait<#error_discriminant> for Error {
            fn from_no(discriminant: #error_discriminant) -> Self {
                match discriminant {
                    #(#from_no_matches)*
                    _ => Self::TODO,
                }
            }

            fn to_no(&self) -> #error_discriminant {
                match *self {
                    #(#to_no_matches)*
                    _ => <#error_discriminant>::MAX,
                }
            }

            fn description(&self) -> &str {
                match *self {
                    #(#desc_matches)*
                    _ => "TODO",
                }
            }

            fn acronym(&self) -> &str {
                match *self {
                    #(#acr_matches)*
                    _ => "TODO",
                }
            }
        }

        impl Into<#error_discriminant> for Error {
            fn into(self) -> #error_discriminant {
                self.to_no()
            }
        }

        impl Default for Error {
            fn default() -> Self {
                Self::TODO
            }
        }

        pub type Result = core::result::Result<ErrorType, Error>;
    }
}
