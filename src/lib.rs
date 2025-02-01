//#![no_std]
pub use embedded_graphics;
use embedded_graphics::{
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::Rectangle,
};

pub mod widgets;
extern crate alloc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResult {
    /// Event processed
    Stop,
    /// Event passed, trying next widget
    Pass,
}

/// затычка
pub struct Event;


#[derive(Clone, Copy)]
pub struct Theme<C: PixelColor> {
    pub background: C,
    pub background2: C,
    pub background3: C,
    pub foreground: C,
    pub foreground2: C,
    pub foreground3: C,
}

impl<C: PixelColor + From<Rgb888>> Theme<C> {
    pub fn hope_diamond() -> Self {
        Self {
            background: Rgb888::new(21, 14, 16).into(),
            background2: Rgb888::new(39, 39, 57).into(),
            background3: Rgb888::new(57, 56, 73).into(),
            foreground: Rgb888::new(119, 136, 140).into(),
            foreground2: Rgb888::new(79, 90, 100).into(),
            foreground3: Rgb888::new(59, 65, 82).into(),
        }
    }

    pub fn binary() -> Self {
        Self {
            background: Rgb888::BLACK.into(),
            background2: Rgb888::BLACK.into(),
            background3: Rgb888::BLACK.into(),
            foreground: Rgb888::WHITE.into(),
            foreground2: Rgb888::WHITE.into(),
            foreground3: Rgb888::WHITE.into(),
        }
    }
}

pub struct UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub draw_target: &'a mut D,
    pub bounds: Rectangle,
    pub theme: Theme<C>,
}

impl<'a, D, C> UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub fn new(draw_target: &'a mut D, bounds: Rectangle, theme: Theme<C>) -> Self {
        Self {
            draw_target,
            bounds,
            theme,
        }
    }
}
