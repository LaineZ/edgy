//! Edgy stylesheet engine - it heavily inspired by CSS.
use alloc::vec::Vec;
use embedded_graphics::{mono_font::MonoFont, prelude::PixelColor};

use crate::Event;

pub type StyleSheet<'a, C> = Vec<StyleRule<'a, C>>;

// TODO: Add more widget selectos
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tag {
    Button,
    ToggleButton,
    Label,
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
    pub modifier: Modifier,
}

impl<'a> Selector<'a> {
    pub const fn new_tag(tag: Tag) -> Self {
        Self {
            kind: SelectorKind::Tag(tag),
            modifier: Modifier::None,
        }
    }

    pub const fn new_root() -> Self {
        Self {
            kind: SelectorKind::Root,
            modifier: Modifier::None,
        }
    }
}

/// Stylesheet struct
#[derive(Copy, Clone, Debug)]
pub struct Style<'a, C: PixelColor> {
    pub background_color: Option<C>,
    pub color: Option<C>,
    pub font: Option<&'a MonoFont<'a>>,
    pub padding: Option<u32>,
}

impl<'a, C: PixelColor> Style<'a, C> {
    pub const fn default() -> Self {
        Self {
            background_color: None,
            color: None,
            font: None,
            padding: None,
        }
    }

    pub fn merge(&mut self, other: Style<'a, C>) {
        if other.background_color.is_some() {
            self.background_color = other.background_color;
        }
        if other.color.is_some() {
            self.color = other.color;
        }

        if other.font.is_some() {
            self.font = other.font;
        }

        if other.padding.is_some() {
            self.padding = other.padding;
        }
    }
}
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

pub fn resolve_style<'a, C: PixelColor + Default>(
    selectors: &[SelectorKind<'a>],
    rules: &[StyleRule<'a, C>],
    modifier: Modifier,
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

        if base_matches && (rule.selector.modifier == Modifier::None || modifier_matches) {
            let specificity = match rule.selector.kind {
                SelectorKind::Root => 0,
                SelectorKind::Tag(_) => 1,
                SelectorKind::Class(_) => 10,
                SelectorKind::Id(_) => 100,
            } + if rule.selector.modifier != Modifier::None {
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
