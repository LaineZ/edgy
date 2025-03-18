use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{renderer::TextRenderer, Alignment, Text},
};

use crate::{Event, EventResult, SystemEvent, Theme, UiContext};

use super::Widget;

/// Generic button style and drawing implementation
#[derive(Clone, Copy)]
pub struct ButtonGeneric<'a, C: PixelColor> {
    pub text_style: Option<MonoTextStyle<'a, C>>,
    pub font: &'a MonoFont<'a>,
    pub style: PrimitiveStyle<C>,
    pub hover_style: PrimitiveStyle<C>,
    pub active_style: PrimitiveStyle<C>,
}

impl<'a, C> ButtonGeneric<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(
        font: &'a MonoFont,
        style: PrimitiveStyle<C>,
        hover_style: PrimitiveStyle<C>,
        active_style: PrimitiveStyle<C>,
    ) -> Self {
        Self {
            font,
            text_style: None,
            style,
            hover_style,
            active_style,
        }
    }

    pub fn size(&mut self, theme: &Theme<C>, text: &str) -> Size {
        self.text_style = Some(MonoTextStyle::new(self.font, theme.foreground));
        let text_size = self
            .text_style
            .unwrap()
            .measure_string(
                text,
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
        text: &str,
    ) {
        let styled_rect = rect.into_styled(self.style);
        let _ = styled_rect.draw(context.draw_target);

        if let Some(style) = self.text_style {
            let text = Text::with_alignment(text, rect.center(), style, Alignment::Center);
            let _ = text.draw(context.draw_target);
        }
    }
}

/// Button widget
pub struct Button<'a, C: PixelColor> {
    base: ButtonGeneric<'a, C>,
    text: String,
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a, C> Button<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new_styled(
        text: String,
        style: ButtonGeneric<'a, C>,
        callback: Box<dyn FnMut() + 'a>,
    ) -> Self {
        Self {
            base: style,
            text,
            callback,
        }
    }

    pub fn new(text: String, font: &'a MonoFont, callback: Box<dyn FnMut() + 'a>) -> Self {
        Self {
            base: ButtonGeneric::new(
                font,
                PrimitiveStyle::default(),
                PrimitiveStyle::default(),
                PrimitiveStyle::default(),
            ),
            text,
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
        // TODO: Refactor, maybe some styling system?
        if self.base.style == PrimitiveStyle::default() {
            self.base.style = PrimitiveStyleBuilder::new()
                .fill_color(context.theme.background)
                .stroke_color(context.theme.background2)
                .stroke_width(1)
                .build();
        }

        if self.base.hover_style == PrimitiveStyle::default() {
            self.base.hover_style = PrimitiveStyleBuilder::new()
                .fill_color(context.theme.background2)
                .stroke_color(context.theme.background2)
                .stroke_width(1)
                .build();
        }

        if self.base.active_style == PrimitiveStyle::default() {
            self.base.active_style = PrimitiveStyleBuilder::new()
                .fill_color(context.theme.background3)
                .stroke_color(context.theme.background2)
                .stroke_width(1)
                .build();
        }

        self.base.size(&context.theme, &self.text)
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
                EventResult::Stop
            }
            Event::Active => {
                self.base.style.fill_color = Some(context.theme.background3);
                (self.callback)();
                EventResult::Stop
            }
            _ => EventResult::Pass,
        }
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.base.draw(context, rect, &self.text);
    }
}
