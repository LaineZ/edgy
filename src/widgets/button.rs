use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle},
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Alignment, Text},
};

use crate::{
    themes::DynamicStyle, Event, EventResult, UiContext
};

use super::{Widget, WidgetEvent};

// TODO: make as field
const PADDING: u32 = 6;

/// Generic button style and drawing implementation
#[derive(Clone, Copy)]
pub struct ButtonGeneric<'a, C: PixelColor> {
    text_style: Option<MonoTextStyle<'a, C>>,
    font: &'a MonoFont<'a>,
    text_alignment: Alignment,
    pub style: DynamicStyle<C>,
}

impl<'a, C> ButtonGeneric<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(font: &'a MonoFont, text_alignment: Alignment, style: DynamicStyle<C>) -> Self {
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
        let _ = styled_rect.draw(&mut context.draw_target);

        if let Some(style) = self.text_style {
            let text = match self.text_alignment {
                Alignment::Left => Text::new(
                    text,
                    Point::new(rect.top_left.x + PADDING as i32, rect.center().y),
                    style,
                ),
                Alignment::Center => {
                    Text::with_alignment(text, rect.center(), style, Alignment::Center)
                }
                Alignment::Right => {
                    let text_width = text.len() as i32 * style.font.character_size.width as i32;
                    let x_pos =
                        rect.top_left.x + rect.size.width as i32 - text_width - PADDING as i32;
                    Text::new(text, Point::new(x_pos, rect.center().y), style)
                }
            };

            let _ = text.draw(&mut context.draw_target);
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
            // wtf
            base: ButtonGeneric::new(font, Alignment::Center, DynamicStyle {
                active: Default::default(),
                drag: Default::default(),
                focus: Default::default(),
                idle: Default::default()
            }),
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
        let style = self.base.style.style(&Event::Idle);
        if style.foreground_color.is_none() && style.background_color.is_none() {
            self.base.style = context.theme.button_style;
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
            Event::Active(_) | Event::Drag(_) => {
                context.focused_element = event_args.id;
                (self.callback)();
                EventResult::Stop
            }
            _ => EventResult::Pass,
        };

        self.base.draw(context, rect, event_args.event, &self.text);
        event_result
    }
}
