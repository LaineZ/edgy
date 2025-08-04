use crate::{style::SelectorKind, EventResult, UiContext};

use super::{Widget, WidgetEvent};
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Widget which wraps any [Drawable].
pub struct Primitive<C: PixelColor, T: Drawable<Color = C> + Dimensions + Transform> {
    primitive: T,
}

impl<C: PixelColor, T: Drawable<Color = C> + Dimensions + Transform> Primitive<C, T> {
    pub fn new(primitive: T) -> Self {
        Self { primitive }
    }
}

impl<'a, D, C, T> Widget<'a, D, C> for Primitive<C, T>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
    T: Drawable<Color = C> + Dimensions + 'a + Transform,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size, _selectors: &[SelectorKind<'a>]) -> Size {
        self.primitive.bounding_box().size
    }

    fn max_size(&mut self) -> Size {
        self.primitive.bounding_box().size
    }

    fn min_size(&mut self) -> Size {
        self.primitive.bounding_box().size
    }

    fn draw(&mut self, context: &mut crate::UiContext<'a, D, C>, _rect: Rectangle, _event_args: WidgetEvent, _selectors: &[SelectorKind<'a>]) -> EventResult {
        self.primitive.translate_mut(_rect.top_left);
        let _ = self.primitive.draw(&mut context.draw_target);
        EventResult::Pass
    }
}
