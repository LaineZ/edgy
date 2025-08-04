//! Edgy stylesheet engine - it heavily inspired by CSS.
use alloc::vec::Vec;
use edgy_style_derive::MergeStyle;
use embedded_graphics::{
    image::ImageRaw,
    mono_font::{mapping, DecorationDimensions, MonoFont, MonoTextStyle},
    prelude::{PixelColor, Size},
    primitives::{PrimitiveStyle, StrokeAlignment},
    text::{self},
};

pub type StyleSheet<'a, C> = Vec<StyleRule<'a, C>>;

pub(crate) const NULL_FONT: MonoFont = MonoFont {
    image: ImageRaw::new(&[], 1),
    character_size: Size::zero(),
    character_spacing: 0,
    baseline: 0,
    strikethrough: DecorationDimensions::new(0, 0),
    underline: DecorationDimensions::new(0, 0),
    glyph_mapping: &mapping::ASCII,
};

// TODO: Add more widget selectors
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tag {
    Button,
    Battery,
    ToggleButton,
    Label,
    Alert,
    SevenSegment,
    Gauge,
    Image,
    Plot,
    Slider
}

/// Selector for widget parts
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Part {
    Main,
    SliderTrack,
    SliderHandle,
    ToggleButtonLightInactive,
    ToggleButtonLightActive,
    /// This is custom selector type for widgets implemented outside the library
    Custom(&'static str),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SelectorKind<'a> {
    Root,
    Tag(Tag),
    Class(&'a str),
    Id(&'a str),
}

/// Style modifier (aka pseudo-class)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Modifier {
    None,
    Focus,
    Active,
    Drag,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Selector<'a> {
    pub kind: SelectorKind<'a>,
    pub part: Part,
    pub modifier: Modifier,
}

impl<'a> Selector<'a> {
    pub const fn new_tag(tag: Tag) -> Self {
        Self {
            kind: SelectorKind::Tag(tag),
            part: Part::Main,
            modifier: Modifier::None,
        }
    }

    pub const fn new_root() -> Self {
        Self {
            kind: SelectorKind::Root,
            part: Part::Main,
            modifier: Modifier::None,
        }
    }
}

/// Stylesheet struct
#[derive(Copy, Clone, Debug, MergeStyle)]
pub struct Style<'a, C: PixelColor> {
    pub background_color: Option<C>,
    pub stroke_color: Option<C>,
    pub accent_color: Option<C>,
    pub stroke_width: Option<u32>,
    pub color: Option<C>,
    pub font: Option<&'a MonoFont<'a>>,
    pub padding: Option<u32>,
    pub line_height: Option<u32>,
    pub text_alignment: Option<text::Alignment>,
}

impl<'a, C: PixelColor> Style<'a, C> {
    pub const fn default() -> Self {
        Self {
            background_color: None,
            stroke_color: None,
            stroke_width: None,
            color: None,
            accent_color: None,
            font: None,
            padding: None,
            line_height: None,
            text_alignment: None,
        }
    }

    pub fn primitive_style(&self) -> PrimitiveStyle<C> {
        let mut style = PrimitiveStyle::new();
        style.fill_color = self.background_color;
        style.stroke_alignment = StrokeAlignment::Inside;
        style.stroke_color = self.stroke_color;
        style.stroke_width = self.stroke_width.unwrap_or_default();
        style
    }

    pub fn character_style(&self) -> MonoTextStyle<'a, C> {
        MonoTextStyle::new(self.font.unwrap_or(&NULL_FONT), self.color.unwrap())
    }
}

#[derive(Clone, Debug)]
pub struct StyleRule<'a, C: PixelColor> {
    pub selector: Selector<'a>,
    pub style: Style<'a, C>,
}

impl<'a, C: PixelColor> StyleRule<'a, C> {
    pub const fn new(selector: Selector<'a>, style: Style<'a, C>) -> Self {
        Self { selector, style }
    }
}

pub struct WidgetStyleContext<'a> {
    pub id: Option<&'a str>,
    pub class: Option<&'a str>,
    pub tag: &'a str,
}

pub fn resolve_style<'a, C: PixelColor>(
    selectors: &[SelectorKind<'a>],
    rules: &[StyleRule<'a, C>],
    modifier: Modifier,
    part: Part,
) -> Style<'a, C> {
    let mut matched: Vec<(&Style<C>, u8)> = Vec::new();

    // match style from root
    if let Some(root_style) = rules.iter().find_map(|rule| {
        if rule.selector.kind == SelectorKind::Root {
            Some(&rule.style)
        } else {
            None
        }
    }) {
        matched.push((root_style, 0));
    }

    for rule in rules {
        let base_matches = selectors.contains(&rule.selector.kind);
        let modifier_matches = rule.selector.modifier == modifier;
        let part_matches = rule.selector.part == part;

        if base_matches
            && (rule.selector.modifier == Modifier::None || modifier_matches)
            && (rule.selector.part == Part::Main || part_matches)
        {
            let specificity = match rule.selector.kind {
                SelectorKind::Root => 0,
                SelectorKind::Tag(_) => 1,
                SelectorKind::Class(_) => 10,
                SelectorKind::Id(_) => 100,
            } + if rule.selector.modifier != Modifier::None { // increase specifity for modifiers
                1
            } else {
                0
            } + if rule.selector.part != Part::Main { // increase specifity for parts
                1
            } else {
                0
            };

            matched.push((&rule.style, specificity));
        }
    }

    matched.sort_by_key(|(_, specificity)| *specificity);

    let mut final_style = Style::default();
    for (style, _) in matched {
        final_style.merge(*style);
    }

    final_style
}
