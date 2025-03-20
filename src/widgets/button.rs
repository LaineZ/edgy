use alloc::{boxed::Box, string::String, sync::Arc};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Alignment, Text},
};

use crate::{
    themes::{NoneStyle, Style},
    Event, EventResult, UiContext,
};

use super::{Widget, WidgetEvent};

// TODO: make as field
const PADDING: u32 = 6;

/// Generic button style and drawing implementation
#[derive(Clone)]
pub struct ButtonGeneric<'a, C: PixelColor> {
    text_style: Option<MonoTextStyle<'a, C>>,
    font: &'a MonoFont<'a>,
    text_alignment: Alignment,
    pub style: Arc<dyn Style<C>>,
}

impl<'a, C> ButtonGeneric<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(font: &'a MonoFont, text_alignment: Alignment, style: Arc<dyn Style<C>>) -> Self {
        Self {
            font,
            style,
            text_alignment,
            text_style: None,
        }
    }

    pub fn size(&mut self, text: &str) -> Size {
        let base_style = self.style.style(&Event::Idle);

        self.text_style = Some(MonoTextStyle::new(
            self.font,
            base_style
                .foreground_color
                .expect("Button must have a foreground color for drawing"),
        ));

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

        Size::new(
            text_size.width + 2 * PADDING,
            text_size.height + 2 * PADDING,
        )
    }

    pub fn draw<D: DrawTarget<Color = C>>(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event: &Event,
        text: &str,
    ) {
        let styled_rect = rect.into_styled(self.style.style(event).into());
        let _ = styled_rect.draw(context.draw_target);

        if let Some(style) = self.text_style {

            let text = match self.text_alignment {
                Alignment::Left => {
                    Text::new(text, Point::new(rect.top_left.x + PADDING as i32, rect.center().y), style)
                },
                Alignment::Center => {
                    Text::with_alignment(text, rect.center(), style, Alignment::Center)
                },
                Alignment::Right => todo!(),
            };

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
            base: ButtonGeneric::new(font, Alignment::Center, NoneStyle::new_arc()),
            text,
            callback,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Button<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        let style = self.base.style.style(&Event::Idle);
        if style.foreground_color.is_none() && style.background_color.is_none() {
            self.base.style = context.theme.button_style.clone();
        }

        self.base.size(&self.text)
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        let event_result = match event_args.event {
            Event::Focus => EventResult::Stop,
            Event::Active => {
                (self.callback)();
                EventResult::Stop
            }
            _ => EventResult::Pass,
        };

        self.base.draw(context, rect, event_args.event, &self.text);
        event_result
    }
}
