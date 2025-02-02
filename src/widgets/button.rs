use alloc::boxed::Box;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{renderer::TextRenderer, Text},
};

use crate::{contains, EventResult, Theme, UiContext};

use super::Widget;

pub struct Button<'a, C: PixelColor> {
    text: &'a str,
    text_style: MonoTextStyle<'a, C>,
    button_style: PrimitiveStyle<C>,
    callback: Box<dyn FnMut() + 'a>,
    rect: Rectangle,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(
        text: &'a str,
        font: &'a MonoFont,
        theme: Theme<C>,
        callback: Box<dyn FnMut() + 'a>,
    ) -> Self {
        Self {
            text,
            text_style: MonoTextStyle::new(font, theme.foreground),
            button_style: PrimitiveStyleBuilder::new()
                .fill_color(theme.background)
                .stroke_color(theme.background2)
                .stroke_width(1)
                .build(),
            callback,
            rect: Rectangle::default(),
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

    fn layout(&mut self, rect: Rectangle) {
        self.rect = rect;
    }

    fn handle_event(&mut self, context: &mut UiContext<'a, D, C>, event: &crate::Event) -> crate::EventResult {
        match event {
            crate::Event::NextWidgetFocus => todo!(),
            crate::Event::PreviousWidgetFocus => todo!(),
            crate::Event::Active(point) => {
                if contains(self.rect, *point) {
                    self.button_style.fill_color = Some(context.theme.foreground2);
                    (self.callback)();
                    return EventResult::Stop
                }

                EventResult::Pass
            },
            crate::Event::Hover(point) => {
                if contains(self.rect, *point) {
                    self.button_style.fill_color = Some(context.theme.background2);
                    return EventResult::Stop
                }

                EventResult::Pass
            },
            _ => EventResult::Pass,
        }
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

        let text_pos =
            rect.center() - Size::new(text_size.width / 2, text_size.height / rect.size.height);
        let text = Text::new(self.text, text_pos, self.text_style);
        let _ = text.draw(context.draw_target);
    }
}
