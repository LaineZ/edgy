use edgy::style::{resolve_style, SelectorKind};
use edgy_style_derive::css;
use edgy::prelude::*;
use edgy::embedded_graphics::prelude::*;
use edgy::embedded_graphics::mono_font::ascii::FONT_4X6;
use edgy::embedded_graphics::pixelcolor::Rgb888;

fn main() {
    let rules = css! {
        Button {
            background_color: Rgb888::CYAN;
            font: &FONT_4X6;
        }
    };

    let rules = resolve_style(&[SelectorKind::Tag(edgy::style::Tag::Button), SelectorKind::Class("danger")], &rules, edgy::style::Modifier::None, edgy::style::Part::Main);
    println!("{:?}", rules);
}
