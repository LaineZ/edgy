use core::char::MAX;

use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{themes::WidgetStyle, EventResult, SystemEvent, UiContext, MAX_SIZE, MIN_SIZE};

use super::{UiBuilder, Widget, WidgetEvent, WidgetObject};

impl<'a, D, C> UiBuilder<'a, D, C> for RootLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObject<'a, D, C>) {
        self.children.push(widget);
    }

    fn finish(self) -> WidgetObject<'a, D, C> {
        WidgetObject::new(Box::new(self))
    }
}

/// Root layout, bascially this is stack layout (literally) puts [Widget]'s in stack and draws it. Difference fron other layout that it's does not implement [UiBuilder] trait, and support only add [WidgetObj]'s directly
pub struct RootLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    children: Vec<WidgetObject<'a, D, C>>,
    dimensions: Rectangle,
}

impl<'a, D, C> RootLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    /// Creates a new `RootLayout` with the specified dimensions.
    /// # Parameters
    /// - `dimensions`: The size and position of the layout on screen.
    pub fn new(dimensions: Rectangle) -> Self {
        Self {
            children: Vec::new(),
            dimensions,
        }
    }

    pub fn add_widget_object(&mut self, obj: WidgetObject<'a, D, C>) {
        self.children.push(obj);
    }

    pub fn finish(self) -> WidgetObject<'a, D, C> {
        WidgetObject::new(Box::new(self) )
    }
}

impl<'a, D, C> Widget<'a, D, C> for RootLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn size(&mut self, _context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        self.dimensions.size
    }

    fn layout(&mut self, context: &mut UiContext<'a, D, C>, _rect: Rectangle) {
        for child in self.children.iter_mut() {
            child.layout(context, self.dimensions);
        }
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        _rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        let mut event_result = EventResult::Pass;

        for child in self.children.iter_mut() {
            if event_result == EventResult::Stop {
                event_result = child.draw(context, &SystemEvent::Idle);
            } else {
                event_result = child.draw(context, event_args.system_event);
            }
        }

        event_result
    }
}
