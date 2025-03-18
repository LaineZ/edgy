use alloc::rc::Rc;
use embedded_graphics::{pixelcolor::Rgb888, prelude::PixelColor};

use crate::widgets::Style;

pub mod hope_diamond;

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
    button_style: Rc<dyn Style<C>>,
    layout_style: Rc<dyn Style<C>>,
    label_style: Rc<dyn Style<C>>,
    pub debug_rect: C,
}
