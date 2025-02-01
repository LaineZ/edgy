use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{renderer::TextRenderer, Text},
};

use crate::{Event, EventResult, UiContext};

use super::Widget;

pub struct Button<'a, C: PixelColor> {
    text: &'a str,
    text_style: MonoTextStyle<'a, C>,
    button_style: PrimitiveStyle<C>,
    text_bounding_box: Rectangle,
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(
        text: &'a str,
        text_style: MonoTextStyle<'a, C>,
        button_style: PrimitiveStyle<C>,
        callback: Box<dyn FnMut() + 'a>,
    ) -> Self {
        Self {
            text,
            text_style,
            button_style,
            callback,
            text_bounding_box: Rectangle::zero(),
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Button<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, _hint: Size) -> Size {
        let text_size = self
            .text_style
            .measure_string(
                self.text,
                Point::zero(),
                embedded_graphics::text::Baseline::Bottom,
            )
            .bounding_box
            .size;

        let padding = 6;
        Size::new(
            text_size.width + 2 * padding,
            text_size.height + 2 * padding,
        )
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let styled_rect = rect.into_styled(self.button_style);
        let _ = styled_rect.draw(context.draw_target);

        let text_size = self
            .text_style
            .measure_string(
                self.text,
                Point::zero(),
                embedded_graphics::text::Baseline::Top,
            )
            .bounding_box
            .size;

        let text_pos = rect.center() - Size::new(text_size.width / 2, text_size.height / rect.size.height);
        let text = Text::new(self.text, text_pos, self.text_style);
        let _ = text.draw(context.draw_target);
    }
}
