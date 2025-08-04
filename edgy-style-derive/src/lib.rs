use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

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
    let input = input.to_string();

    let mut rules = Vec::new();

    for block in input.split('}') {
        if block.trim().is_empty() {
            continue;
        }

        let mut parts = block.splitn(2, '{');
        let selector = parts.next().unwrap().trim();
        let body = parts.next().unwrap_or("").trim();

        let selector_code = parse_selector(selector);
        let style_code = parse_style_block(body);

        rules.push(quote! {
            edgy::style::StyleRule {
                selector: #selector_code,
                style: edgy::style::Style {
                    #(#style_code,)*
                    ..edgy::style::Style::default()
                }
            }
        });
    }

    quote! {
        vec![
            #(#rules),*
        ]
    }
    .into()
}

fn parse_selector(sel: &str) -> proc_macro2::TokenStream {
    let sel = sel.trim();
    let mut part = quote! { edgy::style::Part::Main };
    let mut modifier = quote! { edgy::style::Modifier::None };

    let (base_part, pseudo) = if let Some((a, b)) = sel.split_once("::") {
        (a.trim(), Some(b.trim()))
    } else {
        (sel, None)
    };

    let (base_no_event, base_event) = if let Some((a, b)) = base_part.split_once(':') {
        (a.trim(), Some(b.trim()))
    } else {
        (base_part, None)
    };

    if let Some(evt) = base_event {
        if evt == "focus" {
            modifier = quote! { edgy::style::Modifier::Focus };
        } else if evt == "active" {
            modifier = quote! { edgy::style::Modifier::Active };
        }
    }

    if let Some(pseudo_str) = pseudo {
        let (p, evt) = pseudo_str
            .split_once(':')
            .map(|(a, b)| (a.trim(), Some(b.trim())))
            .unwrap_or((pseudo_str.trim(), None));

        if let Some(evt) = evt {
            if evt == "focus" {
                modifier = quote! { edgy::style::Modifier::Focus };
            }
        }

        part = match p {
            "main" => quote! { edgy::style::Part::Main },
            "slider-track" => quote! { edgy::style::Part::SliderTrack },
            "slider-thumb" => quote! { edgy::style::Part::SliderThumb },
            other => {
                let part_str = syn::LitStr::new(other, proc_macro2::Span::call_site());
                quote! { edgy::style::Part::Custom(#part_str) }
            }
        };
    }

    let kind = if base_no_event.starts_with('#') {
        let id = &base_no_event[1..];
        quote! { edgy::style::SelectorKind::Id(#id) }
    } else if base_no_event.starts_with('.') {
        let class = &base_no_event[1..];
        quote! { edgy::style::SelectorKind::Class(#class) }
    } else {
        let tag_ident = syn::Ident::new(base_no_event, proc_macro2::Span::call_site());
        quote! { edgy::style::SelectorKind::Tag(edgy::style::Tag::#tag_ident) }
    };

    quote! {
        edgy::style::Selector {
            kind: #kind,
            part: #part,
            modifier: #modifier,
        }
    }
}

fn parse_style_block(block: &str) -> Vec<proc_macro2::TokenStream> {
    block
        .lines()
        .filter_map(|line| {
            let line = line.trim().trim_end_matches(';');
            if line.is_empty() {
                return None;
            }

            let mut kv = line.splitn(2, ':');
            let key = kv.next()?.trim();
            let value = kv.next()?.trim();

            let value_ts: proc_macro2::TokenStream = value.parse().ok()?;

            Some(match key {
                "background_color" => quote! { background_color: Some(#value_ts) },
                "font" => quote! { font: Some(#value_ts) },
                _ => {
                    let ident = format_ident!("{}", key);
                    quote! { #ident: #value_ts }
                }
            })
        })
        .collect()
}
