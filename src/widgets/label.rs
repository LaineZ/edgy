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
}

impl<'a, C> Label<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: &'a str, style: MonoTextStyle<'a, C>) -> Self {
        Self {
            text,
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _hint: Size) -> Size {
        let mut size_rect = self
            .style
            .measure_string(
                &self.text,
                Point::zero(),
                embedded_graphics::text::Baseline::Top,
            )
            .bounding_box;

        size_rect.size.height += size_rect.size.height / 2;
        size_rect.size
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let mut position = rect.top_left;
        position.y += self.style.font.character_size.height as i32;
        let text = Text::new(&self.text, position, self.style);
        let _ = text.draw(context.draw_target);
    }
}
