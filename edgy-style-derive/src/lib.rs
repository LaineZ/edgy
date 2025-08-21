use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(MergeStyle)]
pub fn derive_merge_style(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = input.data
    {
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

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let css = input.value();
    let stylesheet = simplecss::StyleSheet::parse(&css);

    let rules = stylesheet.rules.iter().map(|rule| {
        let selector_str = rule.selector.to_string();
        let selector_mods = selector_str.split("::").collect::<Vec<&str>>();
        let is_class = selector_str.starts_with(".");
        let is_id = selector_str.starts_with("#");


        let selector_ident = syn::Ident::new(&selector_str, proc_macro2::Span::call_site());
        let mut color_format = None;

        let declarations = rule.declarations.iter().filter_map(|declaration| {
            let property_str = &declaration.name;

            if *property_str == "color_format" {
                color_format = Some(declaration.value);
                return None;
            }

            let property_ident = syn::Ident::new(property_str, proc_macro2::Span::call_site());

            let value_tokens = match *property_str {
                "background_color" | "stroke_color" | "accent_color" | "color" => {
                    let color_format_ident = syn::Ident::new(
                        color_format
                            .as_ref()
                            .expect("color_format must be defined before color properties"),
                        proc_macro2::Span::call_site(),
                    );
                    let color_ident =
                        syn::Ident::new(&declaration.value, proc_macro2::Span::call_site());
                    quote! { #color_format_ident::#color_ident.into() }
                }
                "stroke_width" | "padding" | "line_height" => {
                    let num: u32 = declaration.value.parse().unwrap();
                    quote! { #num }
                }
                _ => panic!("Unknown property: {}", property_str),
            };

            Some(quote! {
                #property_ident: Some(#value_tokens),
            })
        });

        quote! {
            StyleRule {
                selector: edgy::style::Selector::new_tag(edgy::style::Tag::#selector_ident),
                style: edgy::style::Style {
                    #(#declarations)*
                    ..edgy::style::Style::default()
                }
            }
        }
    });

    TokenStream::from(quote! {
        vec![#(#rules),*]
    })
}
