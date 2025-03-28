use alloc::{boxed::Box, string::String};
use embedded_graphics::{prelude::{DrawTarget, PixelColor}, primitives::Rectangle};

use crate::{themes::WidgetStyle, EventResult, UiContext};

use super::{Widget, WidgetEvent};

pub struct Alert<'a, C: PixelColor> {
    style: WidgetStyle<C>,
    text: String,
    callback: Box<dyn FnMut() + 'a>,
}

impl<'a, C> Alert<'a, C>
where
    C: PixelColor + 'a,
{
    pub fn new(text: String, style: WidgetStyle<C>, callback: Box<dyn FnMut() + 'a>) -> Self {
        Self {
            style,
            text,
            callback,
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Alert<'a, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + 'a,
{
    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        EventResult::Stop
    }
}
