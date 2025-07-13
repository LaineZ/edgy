use alloc::vec::Vec;
use embedded_graphics::{
    mono_font::ascii::FONT_4X6,
    pixelcolor::Rgb888,
    prelude::{PixelColor, RgbColor},
};

use crate::{
    style::{Modifier, Selector, SelectorKind, Style, StyleRule, StyleSheet, Tag},
    Event,
};

const HOPE_DIAMOND_COLOR_BACKGROUND: Rgb888 = Rgb888::new(21, 14, 16);
const HOPE_DIAMOND_COLOR_BACKGROUND2: Rgb888 = Rgb888::new(39, 39, 57);
const HOPE_DIAMOND_COLOR_BACKGROUND3: Rgb888 = Rgb888::new(57, 56, 73);
const HOPE_DIAMOND_COLOR_FOREGROUND: Rgb888 = Rgb888::new(119, 136, 140);
const HOPE_DIAMOND_COLOR_FOREGROUND2: Rgb888 = Rgb888::new(79, 90, 100);
const HOPE_DIAMOND_COLOR_FOREGROUND3: Rgb888 = Rgb888::new(59, 65, 82);
const HOPE_DIAMOND_COLOR_SUCCESS: Rgb888 = Rgb888::new(79, 113, 75);
const HOPE_DIAMOND_COLOR_WARNING: Rgb888 = Rgb888::new(128, 126, 83);

/// Hope diamond theme
pub const HOPE_DIAMOND: [StyleRule<'static, Rgb888>; 4] = [
    // root
    StyleRule::new(
        Selector::new_root(),
        Style {
            background_color: Some(HOPE_DIAMOND_COLOR_BACKGROUND),
            font: Some(&FONT_4X6),
            ..Style::default()
        },
    ),
    // button
    StyleRule::new(
        Selector::new_tag(Tag::Button),
        Style {
            background_color: Some(HOPE_DIAMOND_COLOR_BACKGROUND),
            color: Some(HOPE_DIAMOND_COLOR_FOREGROUND),
            padding: Some(6),
            ..Style::default()
        },
    ),
    // button:Active
    StyleRule::new(
        Selector {
            modifier: Modifier::Active,
            kind: SelectorKind::Tag(Tag::Button),
        },
        Style {
            background_color: Some(HOPE_DIAMOND_COLOR_BACKGROUND2),
            ..Style::default()
        },
    ),
    // button:Focus
    StyleRule::new(
        Selector {
            modifier: Modifier::Focus,
            kind: SelectorKind::Tag(Tag::Button),
        },
        Style {
            background_color: Some(HOPE_DIAMOND_COLOR_BACKGROUND3),
            ..Style::default()
        },
    ),
];
