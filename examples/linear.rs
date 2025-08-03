use edgy::{
    margin,
    themes::{self, DynamicStyle, Theme, WidgetStyle},
    widgets::{
        button::{Button, ButtonGeneric}, linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder}, UiBuilder, WidgetObject
    },
    SystemEvent, UiContext,
};
use edgy_style_derive::css;
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{
    mono_font::{iso_8859_5::FONT_5X8},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    text::{self},
};
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let rules = css! {
        button {
            background_color: Rgb888::BLUE;
        }

        .danger {
            background_color: Rgb888::RED;
        }

        #main:focus {
            font: &FONT_4X6;
        }
    };


    Ok(())
}
