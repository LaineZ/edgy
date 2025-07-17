use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MergeStyle)]
pub fn derive_merge_style(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(ref fields), .. }) = input.data {
        fields
    } else {
        panic!("MergeStyle only supports structs with named fields");
    };

    let merge_fields = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            if other.#field_name.is_some() {
                self.#field_name = other.#field_name;
            }
        }
    });

    let expanded = quote! {
        impl<'a, C: embedded_graphics::prelude::PixelColor> #name<'a, C> {
            pub fn merge(&mut self, other: Self) {
                #(#merge_fields)*
            }
        }
    };

    TokenStream::from(expanded)
}