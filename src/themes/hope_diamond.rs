use alloc::sync::Arc;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{PixelColor, RgbColor},
};

use crate::Event;

use super::{ColorTheme, Style, Theme, WidgetStyle};

const HOPE_DIAMOND_COLORS: ColorTheme = ColorTheme {
    background: Rgb888::new(21, 14, 16),
    background2: Rgb888::new(39, 39, 57),
    background3: Rgb888::new(57, 56, 73),
    foreground: Rgb888::new(119, 136, 140),
    foreground2: Rgb888::new(79, 90, 100),
    foreground3: Rgb888::new(59, 65, 82),
    success: Rgb888::new(79, 113, 75),
    warning: Rgb888::new(128, 126, 83),
    debug_rect: Rgb888::RED,
};

pub struct DefaultButtonStyle;
impl<C: PixelColor + From<Rgb888>> Style<C> for DefaultButtonStyle {
    fn style(&self, event: &Event) -> WidgetStyle<C> {
        let style = WidgetStyle::default()
            .background_color(HOPE_DIAMOND_COLORS.background.into())
            .foreground_color(HOPE_DIAMOND_COLORS.foreground.into())
            .storke(2, HOPE_DIAMOND_COLORS.background2.into())
            .accent_color(HOPE_DIAMOND_COLORS.success.into());

        match event {
            Event::Focus => style.background_color(HOPE_DIAMOND_COLORS.background2.into()),
            Event::Active(_) => style.background_color(HOPE_DIAMOND_COLORS.background3.into()),
            _ => style,
        }
    }
}

pub struct DefaultStyle;
impl<C: PixelColor + From<Rgb888>> Style<C> for DefaultStyle {
    fn style(&self, _event: &Event) -> WidgetStyle<C> {
        WidgetStyle::default().foreground_color(HOPE_DIAMOND_COLORS.foreground.into())
    }
}

pub fn apply<C: PixelColor + From<Rgb888>>() -> Theme<C> {
    Theme {
        button_style: Arc::new(DefaultButtonStyle),
        layout_style: Arc::new(DefaultStyle),
        debug_rect: Rgb888::RED.into(),
        gauge_style: WidgetStyle::default()
            .background_color(HOPE_DIAMOND_COLORS.background.into())
            .foreground_color(HOPE_DIAMOND_COLORS.foreground.into())
            .storke(2, HOPE_DIAMOND_COLORS.foreground.into()),
        modal_style: WidgetStyle::default()
            .background_color(HOPE_DIAMOND_COLORS.background.into())
            .foreground_color(HOPE_DIAMOND_COLORS.foreground.into())
            .storke(2, HOPE_DIAMOND_COLORS.background2.into()),
        plot_style: WidgetStyle::default()
            .background_color(HOPE_DIAMOND_COLORS.background.into())
            .foreground_color(HOPE_DIAMOND_COLORS.background2.into())
            .accent_color(HOPE_DIAMOND_COLORS.foreground.into())
            .storke(2, HOPE_DIAMOND_COLORS.foreground.into()),
        debug_rect_active: Rgb888::GREEN.into(),
    }
}
