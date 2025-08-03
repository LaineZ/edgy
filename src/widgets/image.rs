use super::{Widget, WidgetEvent};
use crate::{style::SelectorKind, EventResult, UiContext};
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Image widget, uses [ImageDrawable] trait for data (raw) you can use any supported image parser for this. Or even, generate image from pixel data! So check the [ImageDrawable] documentation for more info
pub struct Image<'a, I: ImageDrawable> {
    image: &'a I,
}

impl<'a, I> Image<'a, I>
where
    I: ImageDrawable,
{
    pub fn new(image: &'a I) -> Self {
        Self { image }
    }
}

impl<'a, D, I, C> Widget<'a, D, C> for Image<'a, I>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
    I: ImageDrawable<Color = C>,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size, _selectors: &[SelectorKind<'a>]) -> Size {
        self.image.bounding_box().size
    }

    fn max_size(&mut self) -> Size {
        self.image.bounding_box().size
    }

       fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        _event_args: WidgetEvent,
        _selectors: &[SelectorKind<'a>]
    ) -> EventResult {
        let _ = self
            .image
            .draw(&mut context.draw_target.translated(rect.top_left));

        EventResult::Pass
    }
}
