use embedded_graphics::prelude::{DrawTarget, PixelColor, Size};

use crate::UiContext;

pub mod label;
pub mod stack_layout;

pub trait Widget<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn draw(&mut self, ui: &mut UiContext<D, C>);
    fn size(&self) -> Size;
}
