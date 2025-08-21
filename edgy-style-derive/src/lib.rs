use proc_macro::TokenStream;
use quote::quote;
use simplecss::SelectorKindInfo;
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
        let pseudos = rule.selector.pseudo_classes();
        let mut color_format = None;

        let kind = match rule.selector.kind().expect(&format!("{} parsing error", rule.selector.to_string())) {
            SelectorKindInfo::Tag(tag) => {
                let tag_ident = syn::Ident::new(tag, proc_macro2::Span::call_site());
                quote! { edgy::style::SelectorKind::Tag(edgy::style::Tag::#tag_ident) }
            }
            SelectorKindInfo::Id(id) => {
                let id_str = syn::LitStr::new(id, proc_macro2::Span::call_site());
                quote! { edgy::style::SelectorKind::Id(#id_str) }
            }
            SelectorKindInfo::Class(class) => {
                let class_str = syn::LitStr::new(class, proc_macro2::Span::call_site());
                quote! { edgy::style::SelectorKind::Class(#class_str) }
            }
        };

        let modifier = if let Some(pseudo) = pseudos.first() {
            match *pseudo {
                "hover" => quote! { edgy::style::Modifier::Hover },
                "active" => quote! { edgy::style::Modifier::Active },
                "focus" => quote! { edgy::style::Modifier::Focus },
                _ => quote! { edgy::style::Modifier::None },
            }
        } else {
            quote! { edgy::style::Modifier::None }
        };

        let selector_tokens = quote! {
            edgy::style::Selector {
                kind: #kind,
                part: edgy::style::Part::Main,
                modifier: #modifier,
            }
        };

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
                "stroke_width" => {
                    let num: u32 = declaration.value.parse().expect("Number literals must be valid u32 integers");
                    quote! { #num }
                }
                _ => panic!("Unknown property: {}", property_str),
            };

            Some(quote! {
                #property_ident: Some(#value_tokens),
            })
        });

        quote! {
            edgy::style::StyleRule {
                selector: #selector_tokens,
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
