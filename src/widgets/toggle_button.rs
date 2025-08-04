use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use super::{Widget, WidgetEvent, button::ButtonGeneric};
use crate::{
    Event, EventResult, UiContext,
    style::{Part, SelectorKind, Style},
};

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
        context: &mut UiContext<'a, D, C>,
        _hint: Size,
        selectors: &[SelectorKind<'a>],
    ) -> Size {
        self.base.size(&self.text, context, selectors)
    }

    fn is_interactive(&mut self) -> bool {
        true
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
        selectors: &[SelectorKind<'a>],
    ) -> EventResult {
        let event_result = match event_args.event {
            Event::Focus => EventResult::Stop,
            Event::Active(_) => {
                context.focused_element = event_args.id;
                (self.callback)(!self.state);
                EventResult::Stop
            }
            _ => EventResult::Pass,
        };

        self.base.draw(
            context,
            rect,
            &self.text,
            event_args.get_modifier(),
            selectors,
        );

        // TODO: Specify via stylesheet
        let light_size = (rect.size.height / 8).clamp(1, 4);
        let rect_light = Rectangle::new(
            Point::new(
                rect.top_left.x + 1,
                (rect.top_left.y + rect.size.height as i32) - light_size as i32,
            ),
            Size::new(rect.size.width - 2, light_size),
        );

        let part = if self.state {
            Part::ToggleButtonLightActive
        } else {
            Part::ToggleButtonLightInactive
        };

        let resolved_style = context.resolve_style(selectors, event_args.get_modifier(), part);
        let style = resolved_style.primitive_style();
        let _ = rect_light.into_styled(style).draw(&mut context.draw_target);

        event_result
    }
}
