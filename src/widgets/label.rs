use alloc::string::String;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Alignment, Text},
};

use crate::UiContext;

use super::Widget;

pub struct Label<'a, C: PixelColor> {
    text: String,
    style: MonoTextStyle<'a, C>,
    alignment: Alignment,
}

impl<'a, C> Label<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: String, alignment: Alignment, style: MonoTextStyle<'a, C>) -> Self {
        Self {
            text,
            alignment,
            style,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Label<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
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

        match self.alignment {
            Alignment::Left => {
                // do nothing, layout already draws from left
            }
            Alignment::Center => {
                position.x = rect.center().x;
            }
            Alignment::Right => {
                position.x += rect.size.width as i32;
            }
        }

        position.y += self.style.font.character_size.height as i32;
        let text = Text::with_alignment(&self.text, position, self.style, self.alignment);
        let _ = text.draw(context.draw_target);
    }
}
