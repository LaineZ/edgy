use embedded_graphics::{pixelcolor::{Rgb555, Rgb888}, prelude::{PixelColor, RgbColor}};

pub mod hope_diamond;

/// Style for debugging
#[derive(Clone, Copy)]
pub struct DebugStyle<C: PixelColor> {
    pub debug_rect: C,
    pub label_color: C,
    pub debug_rect_active: C,
}


pub fn apply_default_debug_style<C: PixelColor + From<Rgb555> + Default>() -> DebugStyle<C> {
    DebugStyle { debug_rect: Rgb555::RED.into(), label_color: Rgb555::WHITE.into(), debug_rect_active: Rgb555::GREEN.into() }
}