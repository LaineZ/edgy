use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{EventResult, SystemEvent, UiContext};

use super::{Widget, WidgetEvent, WidgetObject};

struct WidgetAndPosition<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    widget_object: WidgetObject<'a, D, C>,
    dimensions: Rectangle,
}

/// Root layout, bascially this is stack layout (literally) puts [Widget]'s in stack and draws it. Difference fron other layout that it's does not implement [UiBuilder] trait, and support only add [WidgetObj]'s directly
pub struct RootLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    children: Vec<WidgetAndPosition<'a, D, C>>
}

impl<'a, D, C> RootLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    /// Creates a new [RootLayout].
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Adds a [WidgetObject] within specified `rect`
    pub fn add_widget_obj(&mut self, widget: WidgetObject<'a, D, C>, rect: Rectangle) {
        self.children.push(WidgetAndPosition { widget_object: widget, dimensions: rect });
    }

    pub fn finish(self) -> WidgetObject<'a, D, C> {
        WidgetObject::new(Box::new(self))
    }
}

impl<'a, D, C> Widget<'a, D, C> for RootLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, _hint: Size) -> Size {
        let mut size = Size::zero();

        for child in self.children.iter_mut() {
            size += child.widget_object.size(context, child.dimensions.size);
        }

        size
    }

    fn layout(&mut self, context: &mut UiContext<'a, D, C>, _rect: Rectangle) {
        for child in self.children.iter_mut() {
            child.widget_object.layout(context, child.dimensions);
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
                event_result = child.widget_object.draw(context, &SystemEvent::Idle);
            } else {
                event_result = child.widget_object.draw(context, event_args.system_event);
            }
        }

        event_result
    }
}
