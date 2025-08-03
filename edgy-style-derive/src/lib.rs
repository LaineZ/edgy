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
            edgy::StyleRule::new(
                #selector_code,
                edgy::style::Style {
                    #(#style_code,)*
                    ..edgy::style::Style::default()
                }
            )
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
    let mut part = quote! { edgy::style::SelectorPart::Main };
    let mut event_class = quote! { edgy::Event::Idle };

    // Разделяем по ::
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
            event_class = quote! { edgy::Event::Focus };
        } else if evt == "hover" {
            event_class = quote! { edgy::Event::Hover };
        }
    }

    let (part_raw, part_event) = if let Some(pseudo_str) = pseudo {
        if let Some((p, evt)) = pseudo_str.split_once(':') {
            if evt == "focus" {
                event_class = quote! { Event::Focus };
            }
            (p.trim(), Some(evt.trim()))
        } else {
            (pseudo_str, None)
        }
    } else {
        ("", None)
    };

    // Определение part
    if !part_raw.is_empty() {
        part = match part_raw {
            "main" => quote! { SelectorPart::Main },
            "slider-track" => quote! { SelectorPart::SliderTrack },
            "slider-thumb" => quote! { SelectorPart::SliderThumb },
            other => quote! { SelectorPart::Custom(#other) },
        };
    }

    // Определение селектора (id / class / tag)
    let kind = if base_no_event.starts_with('#') {
        let id = &base_no_event[1..];
        quote! { edgy::style::SelectorKind::Id(#id) }
    } else if base_no_event.starts_with('.') {
        let class = &base_no_event[1..];
        quote! { edgy::style::SelectorKind::Class(#class) }
    } else {
        let tag = base_no_event;
        quote! { edgy::style::SelectorKind::Tag(#tag) }
    };

    quote! {
        edgy::style::Selector {
            kind: #kind,
            part: #part,
            event_class: #event_class
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

            Some(match key {
                "background_color" => quote! { background_color: Some(#value) },
                "font" => quote! { font: Some(&#value) },
                _ => quote! {}, // expand for more keys
            })
        })
        .collect()
}