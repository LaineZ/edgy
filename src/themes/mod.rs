use embedded_graphics::{pixelcolor::Rgb888, prelude::PixelColor, primitives::PrimitiveStyle};

use crate::{widgets::slider::SliderStyle, Event};

/// dynamic styles for widgets
#[derive(Clone, Copy, Default)]
pub struct DynamicStyle<C: PixelColor> {
    pub idle: WidgetStyle<C>,
    pub focus: WidgetStyle<C>,
    pub active: WidgetStyle<C>,
    pub drag: WidgetStyle<C>,
}

impl<C: PixelColor> DynamicStyle<C> {
    pub fn style(&self, event: &Event) -> WidgetStyle<C> {
        match event {
            Event::Idle => self.idle,
            Event::Focus => self.focus,
            Event::Active(_) => self.active,
            Event::Drag(_) => self.drag,
        }
    }

    pub fn base(&self) -> WidgetStyle<C> {
        self.idle
    }
}

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
#[derive(Clone, Copy)]
pub struct Theme<C: PixelColor> {
    pub button_style: DynamicStyle<C>,
    pub layout_style: DynamicStyle<C>,
    pub slider_style: SliderStyle<C>,
    pub plot_style: WidgetStyle<C>,
    pub gauge_style: WidgetStyle<C>,
    pub modal_style: WidgetStyle<C>,
    pub debug_rect: C,
    pub label_color: C,
    pub debug_rect_active: C,
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

impl<C: PixelColor> Into<DynamicStyle<C>> for WidgetStyle<C> {
    fn into(self) -> DynamicStyle<C> {
        DynamicStyle {
            idle: self,
            focus: self,
            active: self,
            drag: self,
        }
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
