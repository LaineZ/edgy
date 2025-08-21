use edgy::embedded_graphics::mono_font::ascii::FONT_4X6;
use edgy::embedded_graphics::pixelcolor::Rgb888;
use edgy::embedded_graphics::prelude::*;
use edgy::prelude::*;
use edgy::style::StyleSheet;
use edgy::style::{resolve_style, SelectorKind};
use edgy_style_derive::css;
use edgy::style::StyleRule;

fn main() {
    let rules: StyleSheet<Rgb888> = css!(
        r#"
        Button {
            color_format: Rgb888;
            background_color: RED;
            stroke_width: 2;
        }

        Button::focus {
            background_color: BLUE:
        }

        Label {
            color_format: Rgb888;
            color: WHITE;
        }
    "#
    );

    println!("RULES: {:?}", rules);

    let rules = resolve_style(
        &[
            SelectorKind::Tag(edgy::style::Tag::Button),
        ],
        &rules,
        edgy::style::Modifier::Focus,
        edgy::style::Part::Main,
    );
    println!("{:?}", rules);
}
