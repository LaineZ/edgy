use alloc::rc::Rc;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{PixelColor, RgbColor},
};

use crate::{
    widgets::{self, Style, WidgetStyle},
    Event,
};

use super::{ColorTheme, Theme};

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

struct DefaultButtonStyle;
impl<C: PixelColor + From<Rgb888>> Style<C>
    for DefaultButtonStyle
{
    fn style(&self, event: &Event) -> widgets::WidgetStyle<C> {
        let style = WidgetStyle::default()
            .background_color(HOPE_DIAMOND_COLORS.background.into())
            .foreground_color(HOPE_DIAMOND_COLORS.foreground.into());

        match event {
            Event::Focus => style.background_color(HOPE_DIAMOND_COLORS.background2.into()),
            Event::Active => style.background_color(HOPE_DIAMOND_COLORS.background3.into()),
            _ => style,
        }
    }
}

struct DefaultStyle;
impl<C: PixelColor + From<Rgb888>> Style<C> for DefaultStyle {
    fn style(&self, _event: &Event) -> widgets::WidgetStyle<C> {
        WidgetStyle::default().foreground_color(HOPE_DIAMOND_COLORS.foreground.into())
    }
}

pub fn hope_diamond<C: PixelColor + From<Rgb888>>() -> Theme<C> {
    Theme {
        button_style: Rc::new(DefaultButtonStyle),
        layout_style: Rc::new(DefaultStyle),
        label_style: Rc::new(DefaultStyle),
        debug_rect: Rgb888::RED.into(),
    }
}
