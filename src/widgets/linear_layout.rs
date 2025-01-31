use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{Event, EventResult, UiContext};

use super::{UiBuilder, Widget, WidgetObj};

pub struct LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub children: Vec<WidgetObj<'a, D, C>>,
    pub direction: LayoutDirection,
}

impl<'a, D, C> Default for LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            children: Vec::new(),
            direction: LayoutDirection::Vertical,
        }
    }
}

impl<'a, D, C> UiBuilder<'a, D, C> for LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObj<'a, D, C>) {
        self.children.push(widget);
    }

    fn finish(self) -> WidgetObj<'a, D, C> {
        WidgetObj {
            widget: Box::new(LinearLayout {
                direction: self.direction,
                children: self.children,
            }),
        }
    }
}

pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

pub struct LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    children: Vec<WidgetObj<'a, D, C>>,
    direction: LayoutDirection,
}

impl<'a, D, C> Widget<'a, D, C> for LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn layout(&mut self, hint: Size) -> Size {
        let mut width = 0;
        let mut height = 0;

        for child in &mut self.children {
            let child_size = child.layout(hint);

            match self.direction {
                LayoutDirection::Horizontal => {
                    width += child_size.width;
                    height = height.max(child_size.height);
                }
                LayoutDirection::Vertical => {
                    height += child_size.height;
                    width = width.max(child_size.width);
                }
            }
        }

        Size::new(width.min(hint.width), height.min(hint.height))
    }

    fn handle_event(&mut self, event: &Event) -> EventResult {
        let mut result = EventResult::Pass;

        for child in &mut self.children {
            result = child.handle_event(event);
            if result == EventResult::Stop {
                break;
            }
        }

        result
    }

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let mut offset = Point::zero();

        for child in &mut self.children {
            let child_size = child.layout(Size::new(rect.size.width, rect.size.height));
            let limited_size = Size::new(
                child_size.width.min(rect.size.width),
                child_size.height.min(rect.size.height),
            );

            let child_rect = Rectangle::new(rect.top_left + offset, limited_size);
            child.draw(context, child_rect);

            match self.direction {
                LayoutDirection::Horizontal => {
                    offset.x += limited_size.width as i32;
                }
                LayoutDirection::Vertical => {
                    offset.y += limited_size.height as i32;
                }
            }
        }
    }
}
