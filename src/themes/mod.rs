use alloc::sync::Arc;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::PixelColor,
    primitives::PrimitiveStyle,
};

use crate::Event;

pub mod hope_diamond;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) struct ColorTheme {
    /// Primary background
    pub(crate) background: Rgb888,
    pub(crate) background2: Rgb888,
    pub(crate) background3: Rgb888,
    /// Primary foreground
    pub(crate) foreground: Rgb888,
    pub(crate) foreground2: Rgb888,
    pub(crate) foreground3: Rgb888,
    pub(crate) debug_rect: Rgb888,
    pub(crate) success: Rgb888,
    pub(crate) warning: Rgb888,
}

/// Theme struct. You can freely create own themes
#[derive(Clone)]
pub struct Theme<C: PixelColor> {
    // using arc due to thread safety, since rust ALWAYS assume static code as multi threaded
    pub button_style: Arc<dyn Style<C>>,
    pub layout_style: Arc<dyn Style<C>>,
    pub plot_style: WidgetStyle<C>,
    pub gauge_style: WidgetStyle<C>,
    pub modal_style: WidgetStyle<C>,
    pub debug_rect: C,
}

/// Base style for any widget, basically any widget can have this style
#[derive(Clone, Copy)]
pub struct WidgetStyle<C: PixelColor> {
    /// Accent (active) color of widget
    pub accent_color: Option<C>,
    /// Foreground color for widget elements
    pub foreground_color: Option<C>,
    /// Background color for widget
    pub background_color: Option<C>,
    /// Border color
    pub stroke_color: Option<C>,
    /// Border width
    pub stroke_width: u32,
}

impl<C: PixelColor> Default for WidgetStyle<C> {
    fn default() -> Self {
        Self {
            accent_color: Default::default(),
            foreground_color: Default::default(),
            background_color: Default::default(),
            stroke_color: Default::default(),
            stroke_width: Default::default(),
        }
    }
}

impl<C: PixelColor> WidgetStyle<C> {
    pub fn foreground_color(mut self, color: C) -> Self {
        self.foreground_color = Some(color);
        self
    }

    pub fn accent_color(mut self, color: C) -> Self {
        self.accent_color = Some(color);
        self
    }

    pub fn background_color(mut self, color: C) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn storke(mut self, width: u32, color: C) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }
}

impl<C: PixelColor> From<WidgetStyle<C>> for PrimitiveStyle<C> {
    fn from(val: WidgetStyle<C>) -> Self {
        let mut style = PrimitiveStyle::<C>::default();
        style.fill_color = val.background_color;
        style.stroke_color = val.stroke_color;
        style.stroke_width = val.stroke_width;

        style
    }
}

impl<C: PixelColor> From<PrimitiveStyle<C>> for WidgetStyle<C> {
    fn from(value: PrimitiveStyle<C>) -> Self {
        Self {
            background_color: value.fill_color,
            stroke_color: value.stroke_color,
            stroke_width: value.stroke_width,
            ..Default::default()
        }
    }
}

pub struct NoneStyle;

impl<C: PixelColor> Style<C> for NoneStyle {
    fn style(&self, _event: &Event) -> WidgetStyle<C> {
        WidgetStyle::default()
    }
}

impl NoneStyle {
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self)
    }
}

/// Base primitive style for widget
pub trait Style<C: PixelColor> {
    /// Specifies style that depends on [Event]
    fn style(&self, event: &Event) -> WidgetStyle<C>;

    /// Base style of widget for non-interactive widgets or default state for interactive one
    fn base(&self) -> WidgetStyle<C> {
        self.style(&Event::Idle)
    }
}
