use alloc::boxed::Box;
use embedded_graphics::{
    mono_font::MonoFont,
    prelude::*,
    primitives::Rectangle,
};

use crate::{Event, EventResult, SystemEvent, UiContext};
use super::{button::ButtonGeneric, Widget};

pub struct ToggleButton<'a, C: PixelColor> {
    base: ButtonGeneric<'a, C>,
    state: bool,
    callback: Box<dyn FnMut(bool) + 'a>,
}

impl<'a, C> ToggleButton<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: &'a str, font: &'a MonoFont, state: bool, callback: Box<dyn FnMut(bool) + 'a>) -> Self {
        Self {
            base: ButtonGeneric::new(text, font),
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
        system_event: &SystemEvent,
        event: &Event,
    ) -> crate::EventResult {
        match event {
            Event::Focus => {
                self.base.style.fill_color = Some(context.theme.background2);
                return EventResult::Stop;
            },
            Event::Active => {
                self.base.style.fill_color = Some(context.theme.background3);
                (self.callback)(!self.state);
                context.consume_event(system_event);
                return EventResult::Stop;
            },
            _ => {
                EventResult::Pass
            }
        }
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.base.draw(context, rect);
    }
}
