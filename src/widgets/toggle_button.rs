use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::MonoFont,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Alignment,
};

use super::{button::ButtonGeneric, Widget, WidgetEvent};
use crate::{themes::DynamicStyle, Event, EventResult, UiContext};

/// Toggle button (Korry-like switches)
pub struct ToggleButton<'a, C: PixelColor> {
    base: ButtonGeneric<'a, C>,
    text: String,
    state: bool,
    callback: Box<dyn FnMut(bool) + 'a>,
}

impl<'a, C> ToggleButton<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new_styled(
        text: String,
        style: ButtonGeneric<'a, C>,
        state: bool,
        callback: Box<dyn FnMut(bool) + 'a>,
    ) -> Self {
        Self {
            base: style,
            text,
            state,
            callback,
        }
    }

    pub fn new(
        text: String,
        font: &'a MonoFont,
        state: bool,
        callback: Box<dyn FnMut(bool) + 'a>,
    ) -> Self {
        Self {
            // wtf
            base: ButtonGeneric::new(
                font,
                Alignment::Center,
                DynamicStyle {
                    active: Default::default(),
                    drag: Default::default(),
                    focus: Default::default(),
                    idle: Default::default(),
                },
            ),
            text,
            state,
            callback,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for ToggleButton<'a, C>
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
        let style = self.base.style.style(event_args.event);

        let event_result = match event_args.event {
            Event::Focus => EventResult::Stop,
            Event::Active(_) => {
                context.focused_element = event_args.id;
                (self.callback)(!self.state);
                EventResult::Stop
            }
            _ => EventResult::Pass,
        };

        self.base.draw(context, rect, event_args.event, &self.text);
        let light_size = (rect.size.height / 8).clamp(1, 4);
        let rect_light = Rectangle::new(
            Point::new(
                rect.top_left.x + 1,
                (rect.top_left.y + rect.size.height as i32) - light_size as i32,
            ),
            Size::new(rect.size.width - 2, light_size),
        );
        if self.state {
            let _ = rect_light
                .into_styled(PrimitiveStyle::with_fill(
                    style
                        .accent_color
                        .expect("Toggle button must have a accent color for drawing"),
                ))
                .draw(&mut context.draw_target);
        } else {
            if let Some(foreground_color) = style.foreground_color {
                let _ = rect_light
                    .into_styled(PrimitiveStyle::with_fill(foreground_color))
                    .draw(&mut context.draw_target);
            }
        }

        event_result
    }
}
