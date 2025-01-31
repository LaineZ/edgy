use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Text},
};

use crate::UiContext;

use super::Widget;

pub struct Label<'a, C: PixelColor> {
    text: &'a str,
    style: MonoTextStyle<'a, C>,
    position: Point,
}

impl<'a, C> Label<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: &'a str, style: MonoTextStyle<'a, C>) -> Self {
        Self {
            text,
            style,
            position: Point::default(),
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn layout(&mut self, _hint: Size) -> Size {
        let size_rect = self
            .style
            .measure_string(
                &self.text,
                self.position,
                embedded_graphics::text::Baseline::Middle,
            )
            .bounding_box;
        size_rect.size
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.position = rect.top_left;
        let text = Text::new(&self.text, rect.top_left, self.style);
        let _ = text.draw(context.draw_target);
    }
}
