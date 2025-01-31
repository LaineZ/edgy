//#![no_std]
pub use embedded_graphics;
use embedded_graphics::{prelude::*, primitives::Rectangle};

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

pub struct UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub draw_target: &'a mut D,
    pub bounds: Rectangle,
}

impl<'a, D, C> UiContext<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub fn new(draw_target: &'a mut D, bounds: Rectangle) -> Self {
        Self {
            draw_target,
            bounds,
        }
    }
}