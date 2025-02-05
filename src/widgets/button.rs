use alloc::boxed::Box;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{renderer::TextRenderer, Alignment, Text},
};

use crate::{contains, Event, EventResult, SystemEvent, UiContext};

use super::Widget;

pub struct Button<'a, C: PixelColor> {
    text: &'a str,
    text_style: Option<MonoTextStyle<'a, C>>,
    font: &'a MonoFont<'a>,
    button_style: PrimitiveStyle<C>,
    callback: Box<dyn FnMut() + 'a>,
    is_hovered: bool,
    rect: Rectangle,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: &'a str, font: &'a MonoFont, callback: Box<dyn FnMut() + 'a>) -> Self {
        Self {
            is_hovered: false,
            text,
            font,
            text_style: None,
            button_style: PrimitiveStyleBuilder::new().build(),
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
    fn size(&mut self, context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        self.text_style = Some(MonoTextStyle::new(self.font, context.theme.foreground));
        self.button_style = PrimitiveStyleBuilder::new()
            .fill_color(context.theme.background)
            .stroke_color(context.theme.background2)
            .stroke_width(1)
            .build();

        let text_size = self
            .text_style
            .unwrap()
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

    fn layout(&mut self, _context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.rect = rect;
    }


    fn is_interactive(&mut self) -> bool {
        true
    }

    fn handle_event(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        _system_event: &SystemEvent,
        event: &Event,
    ) -> crate::EventResult {
        match event {
            Event::Focus => {
                self.is_hovered = true;
                self.button_style.fill_color = Some(context.theme.background2);
                return EventResult::Stop;
            },
            Event::Active => {
                self.button_style.fill_color = Some(context.theme.background3);
                (self.callback)();
                return EventResult::Stop;
            },
            _ => {
                EventResult::Pass
            }
        }
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let styled_rect = rect.into_styled(self.button_style);
        let _ = styled_rect.draw(context.draw_target);

        let text = Text::with_alignment(
            self.text,
            rect.center(),
            self.text_style.unwrap(),
            Alignment::Center,
        );
        let _ = text.draw(context.draw_target);
    }
}
