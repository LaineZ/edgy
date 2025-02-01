use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{Event, EventResult, UiContext};

use super::{UiBuilder, Widget, WidgetObj};

#[derive(PartialEq, Clone, Copy)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
    HorizontalFill,
    VerticalFill,
}

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

fn compute_child_size(
    direction: LayoutDirection,
    child_size: Size,
    hint: Size,
    children_count: usize,
) -> Size {
    match direction {
        LayoutDirection::Horizontal | LayoutDirection::Vertical => child_size,
        LayoutDirection::HorizontalFill => {
            Size::new(hint.width / children_count as u32, child_size.height)
        }
        LayoutDirection::VerticalFill => {
            Size::new(child_size.width, hint.height / children_count as u32)
        }
    }
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
    fn size(&mut self, hint: Size) -> Size {
        let children_count = self.children.len();
        let mut width = 0;
        let mut height = 0;

        for child in &mut self.children {
            let child_size =
                compute_child_size(self.direction, child.size(hint), hint, children_count);

            match self.direction {
                LayoutDirection::Horizontal | LayoutDirection::HorizontalFill => {
                    width += child_size.width;
                    height = height.max(child_size.height);
                }
                LayoutDirection::Vertical | LayoutDirection::VerticalFill => {
                    width = width.max(child_size.width);
                    height += child_size.height;
                }
            }
        }

        match self.direction {
            LayoutDirection::HorizontalFill => {
                width = hint.width;
            }
            LayoutDirection::VerticalFill => {
                height = hint.height;
            }
            _ => {}
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

        let children_count = self.children.len() as u32;

        for child in &mut self.children {
            let child_size = match self.direction {
                LayoutDirection::Horizontal | LayoutDirection::Vertical => {
                    child.size(Size::new(rect.size.width, rect.size.height))
                }
                LayoutDirection::HorizontalFill => {
                    Size::new(rect.size.width / children_count, rect.size.height)
                }
                LayoutDirection::VerticalFill => {
                    Size::new(rect.size.width, rect.size.height / children_count)
                }
            };

            let child_rect = Rectangle::new(rect.top_left + offset, child_size);
            child.draw(context, child_rect);

            match self.direction {
                LayoutDirection::Horizontal | LayoutDirection::HorizontalFill => {
                    offset.x += child_size.width as i32;
                }
                LayoutDirection::Vertical | LayoutDirection::VerticalFill => {
                    offset.y += child_size.height as i32;
                }
            }
        }
    }
}
