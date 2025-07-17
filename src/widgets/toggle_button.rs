use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use super::{button::ButtonGeneric, Widget, WidgetEvent};
use crate::{style::Style, Event, EventResult, UiContext};

/// Toggle button (Korry-like switches)
pub struct ToggleButton<'a> {
    base: ButtonGeneric,
    text: String,
    state: bool,
    callback: Box<dyn FnMut(bool) + 'a>,
}

impl<'a> ToggleButton<'a> {
    pub fn new(text: String, state: bool, callback: Box<dyn FnMut(bool) + 'a>) -> Self {
        Self {
            // wtf
            base: ButtonGeneric::new(),
            text,
            state,
            callback,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for ToggleButton<'a>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn size(
        &mut self,
        _context: &mut UiContext<'a, D, C>,
        _hint: Size,
        resolved_style: &Style<'a, C>,
    ) -> Size {
        self.base.size(&self.text, resolved_style)
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
        resolved_style: &Style<'a, C>,
    ) -> EventResult {
        let style = resolved_style.primitive_style();

        let event_result = match event_args.event {
            Event::Focus => EventResult::Stop,
            Event::Active(_) => {
                context.focused_element = event_args.id;
                (self.callback)(!self.state);
                EventResult::Stop
            }
            _ => EventResult::Pass,
        };

        self.base.draw(context, rect, resolved_style, &self.text);
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
                    resolved_style
                        .accent_color.unwrap_or(style.stroke_color.unwrap())
                ))
                .draw(&mut context.draw_target);
        } else {
            if let Some(foreground_color) = resolved_style.color {
                let _ = rect_light
                    .into_styled(PrimitiveStyle::with_fill(foreground_color))
                    .draw(&mut context.draw_target);
            }
        }

        event_result
    }
}
