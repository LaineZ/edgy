use alloc::boxed::Box;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{renderer::TextRenderer, Alignment, Text},
};

use crate::{Event, EventResult, SystemEvent, Theme, UiContext};

use super::Widget;

/// Generic button style and drawing implementation
pub struct ButtonGeneric<'a, C: PixelColor> {
    pub text: &'a str,
    pub text_style: Option<MonoTextStyle<'a, C>>,
    pub font: &'a MonoFont<'a>,
    pub style: PrimitiveStyle<C>,
    pub rect: Rectangle,
}

impl<'a, C> ButtonGeneric<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: &'a str, font: &'a MonoFont) -> Self {
        Self {
            text,
            font,
            text_style: None,
            style: PrimitiveStyleBuilder::new().build(),
            rect: Rectangle::default(),
        }
    }

    pub fn size(&mut self, theme: Theme<C>) -> Size {
        self.text_style = Some(MonoTextStyle::new(self.font, theme.foreground));
        self.style = PrimitiveStyleBuilder::new()
            .fill_color(theme.background)
            .stroke_color(theme.background2)
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

    pub fn draw<D: DrawTarget<Color = C>>(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
    ) {
        let styled_rect = rect.into_styled(self.style);
        let _ = styled_rect.draw(context.draw_target);

        if let Some(style) = self.text_style {
            let text = Text::with_alignment(self.text, rect.center(), style, Alignment::Center);
            let _ = text.draw(context.draw_target);
        }
    }
}

/// Button widget
pub struct Button<'a, C: PixelColor> {
    base: ButtonGeneric<'a, C>,
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: &'a str, font: &'a MonoFont, callback: Box<dyn FnMut() + 'a>) -> Self {
        Self {
            base: ButtonGeneric::new(text, font),
            callback,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Button<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        self.base.size(context.theme)
    }

    fn layout(&mut self, _context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.base.rect = rect;
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
                self.base.style.fill_color = Some(context.theme.background2);
                return EventResult::Stop;
            }
            Event::Active => {
                self.base.style.fill_color = Some(context.theme.background3);
                (self.callback)();
                return EventResult::Stop;
            }
            _ => EventResult::Pass,
        }
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.base.draw(context, rect);
    }
}
