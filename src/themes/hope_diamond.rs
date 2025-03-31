use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{PixelColor, RgbColor, Size},
};

use crate::widgets::slider::SliderStyle;

use super::{ColorTheme, DynamicStyle, Theme, WidgetStyle};

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

pub fn apply<C: PixelColor + From<Rgb888> + Default>() -> Theme<C> {
    let button_style = WidgetStyle::default()
        .background_color(HOPE_DIAMOND_COLORS.background.into())
        .foreground_color(HOPE_DIAMOND_COLORS.foreground.into())
        .storke(2, HOPE_DIAMOND_COLORS.background2.into())
        .accent_color(HOPE_DIAMOND_COLORS.success.into());

    Theme {
        button_style: DynamicStyle {
            idle: button_style,
            focus: button_style.background_color(HOPE_DIAMOND_COLORS.background2.into()),
            active: button_style.background_color(HOPE_DIAMOND_COLORS.background3.into()),
            drag: button_style.background_color(HOPE_DIAMOND_COLORS.background3.into()),
        },
        slider_style: SliderStyle::new(
            button_style.into(),
            button_style.into(),
            2,
            Size::new(2, 4),
        ),
        layout_style: DynamicStyle::default(),
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
