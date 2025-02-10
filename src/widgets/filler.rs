use crate::UiContext;

use super::Widget;
use embedded_graphics::{prelude::*, primitives::Rectangle};

pub enum FillStrategy {
    Vertical,
    Horizontal,
    Both,
}

/// Widget space filler widget
pub struct Filler {
    fill: FillStrategy,
}

impl Filler {
    pub fn new(fill: FillStrategy) -> Self {
        Self { fill }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Filler
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        match self.fill {
            FillStrategy::Vertical => Size::new(0, hint.height),
            FillStrategy::Horizontal => Size::new(hint.width, 0),
            FillStrategy::Both => hint,
        }
    }

    fn draw(&mut self, _context: &mut crate::UiContext<'a, D, C>, _rect: Rectangle) {
        // do nothing...
    }
}
