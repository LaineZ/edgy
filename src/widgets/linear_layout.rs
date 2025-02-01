use std::u32;

use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{Event, EventResult, UiContext};

use super::{UiBuilder, Widget, WidgetObj};

#[derive(PartialEq, Clone, Copy)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LayoutAlignment {
    Start,
    Center,
    End,
    Stretch,
}

pub struct LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub children: Vec<WidgetObj<'a, D, C>>,
    pub alignment: LayoutAlignment,
    pub direction: LayoutDirection,
    pub min_size: Size,
    pub max_size: Size,
}

impl<'a, D, C> LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub fn min_size(mut self, min_size: Size) -> Self {
        self.min_size = min_size;
        self
    }

    pub fn max_size(mut self, max_size: Size) -> Self {
        self.max_size = max_size;
        self
    }

    pub fn aligment(mut self, alignment: LayoutAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, D, C> Default for LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            children: Vec::new(),
            alignment: LayoutAlignment::Start,
            direction: LayoutDirection::Vertical,
            min_size: Size::zero(),
            max_size: Size::new(u32::MAX, u32::MAX),
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
                aligment: self.alignment,
                min_size: self.min_size,
                max_size: self.max_size,
            }),
        }
    }
}

fn compute_child_size(
    direction: LayoutDirection,
    alignment: LayoutAlignment,
    child_size: Size,
    hint: Size,
    children_count: usize,
) -> Size {
    if alignment != LayoutAlignment::Stretch {
        return child_size;
    } else {
        return match direction {
            LayoutDirection::Horizontal => {
                Size::new(hint.width / children_count as u32, child_size.height)
            }
            LayoutDirection::Vertical => {
                Size::new(child_size.width, hint.height / children_count as u32)
            }
        };
    }
}

pub struct LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    children: Vec<WidgetObj<'a, D, C>>,
    direction: LayoutDirection,
    aligment: LayoutAlignment,
    min_size: Size,
    max_size: Size,
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
            let child_size = compute_child_size(
                self.direction,
                self.aligment,
                child.size(hint),
                hint,
                children_count,
            );

            match self.direction {
                LayoutDirection::Horizontal => {
                    width += child_size.width;
                    height = height.max(child_size.height);
                }
                LayoutDirection::Vertical => {
                    width = width.max(child_size.width);
                    height += child_size.height;
                }
            }
        }

        if self.aligment == LayoutAlignment::Stretch {
            match self.direction {
                LayoutDirection::Horizontal => {
                    width = hint.width;
                }
                LayoutDirection::Vertical => {
                    height = hint.height;
                }
            }
        }

        Size::new(
            width
                .min(hint.width)
                .clamp(self.min_size.width, self.max_size.width),
            height
                .min(hint.height)
                .clamp(self.min_size.height, self.max_size.height),
        )
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
        let children_count = self.children.len() as u32;

        let total_length = match self.direction {
            LayoutDirection::Horizontal => {
                let mut total = 0;
                for child in &mut self.children {
                    let child_size = child.size(Size::new(rect.size.width, rect.size.height));
                    total += child_size.width;
                }
                total
            }
            LayoutDirection::Vertical => {
                let mut total = 0;
                for child in &mut self.children {
                    let child_size = child.size(Size::new(rect.size.width, rect.size.height));
                    total += child_size.height;
                }
                total
            }
        };

        let mut offset = match (self.direction, self.aligment) {
            (LayoutDirection::Horizontal, LayoutAlignment::Center) => {
                let free_space = rect.size.width - total_length;
                Point::new((free_space / 2) as i32, 0)
            }

            (LayoutDirection::Horizontal, LayoutAlignment::End) => {
                let free_space = rect.size.width - total_length;
                Point::new(free_space as i32, 0)
            }

            (LayoutDirection::Vertical, LayoutAlignment::Center) => {
                let free_space = rect.size.height - total_length;
                Point::new(0, (free_space / 2) as i32)
            }

            (LayoutDirection::Vertical, LayoutAlignment::End) => {
                let free_space = rect.size.height - total_length;
                Point::new(0, free_space as i32)
            }

            _ => Point::zero(),
        };

        for child in &mut self.children {
            let child_size = if self.aligment != LayoutAlignment::Stretch {
                child.size(Size::new(rect.size.width, rect.size.height))
            } else {
                match self.direction {
                    LayoutDirection::Horizontal => {
                        Size::new(rect.size.width / children_count, rect.size.height)
                    }
                    LayoutDirection::Vertical => {
                        Size::new(rect.size.width, rect.size.height / children_count)
                    }
                }
            };

            let mut child_rect = Rectangle::new(rect.top_left + offset, child_size);
            child_rect.size = Size::new(
                child_rect
                    .size
                    .width
                    .clamp(self.min_size.width, self.max_size.width),
                child_rect
                    .size
                    .height
                    .clamp(self.min_size.height, self.max_size.height),
            );
            child.draw(context, child_rect);

            match self.direction {
                LayoutDirection::Horizontal => {
                    offset.x += child_size.width as i32;
                }
                LayoutDirection::Vertical => {
                    offset.y += child_size.height as i32;
                }
            }
        }
    }
}
