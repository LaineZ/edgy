use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::{Event, EventResult, SystemEvent, UiContext};

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
        WidgetObj::new(Box::new(LinearLayout {
            direction: self.direction,
            children: self.children,
            aligment: self.alignment,
            min_size: self.min_size,
            max_size: self.max_size,
        }))
    }
}

fn compute_child_size(
    direction: LayoutDirection,
    alignment: LayoutAlignment,
    child_size: Size,
    hint: Size,
    children_count: usize,
) -> Size {

    return match alignment {
        LayoutAlignment::Stretch => {
            match direction {
                LayoutDirection::Horizontal => {
                    Size::new(hint.width / children_count as u32, child_size.height)
                }
                LayoutDirection::Vertical => {
                    Size::new(child_size.width, hint.height / children_count as u32)
                }
            }
        },
        _ => {
            return child_size;
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
    aligment: LayoutAlignment,
    min_size: Size,
    max_size: Size,
}
impl<'a, D, C> Widget<'a, D, C> for LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        let children_count = self.children.len();
        let mut width = 0;
        let mut height = 0;

        for child in &mut self.children {
            let child_size = compute_child_size(
                self.direction,
                self.aligment,
                child.size(context, hint),
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

        Size::new(width.min(hint.width), height.min(hint.height))
    }

    fn max_size(&mut self) -> Size {
        self.max_size
    }

    fn min_size(&mut self) -> Size {
        self.min_size
    }

    fn handle_event(&mut self, context: &mut UiContext<'a, D, C>, system_event: &SystemEvent, _event: &Event) -> EventResult {
        let mut result = EventResult::Pass;

        for child in &mut self.children {
            result = child.handle_event(context, system_event);
            if result == EventResult::Stop {
                break;
            }
        }

        result
    }

    fn layout(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let children_count = self.children.len() as u32;

        let total_length = match self.direction {
            LayoutDirection::Horizontal => {
                let mut total = 0;
                for child in &mut self.children {
                    let child_size = child.size(context, Size::new(rect.size.width, rect.size.height));
                    total += child_size.width;
                }
                total
            }
            LayoutDirection::Vertical => {
                let mut total = 0;
                for child in &mut self.children {
                    let child_size = child.size(context, Size::new(rect.size.width, rect.size.height));
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
                let free_space = rect.size.height.saturating_sub(total_length);
                Point::new(0, free_space as i32)
            }

            _ => Point::zero(),
        };


        for child in &mut self.children {
            let child_bounds = child.calculate_bound_sizes(rect.size);

            let child_size = if self.aligment != LayoutAlignment::Stretch {
                child.size(context, child_bounds)
            } else {
                match self.direction {
                    LayoutDirection::Horizontal => {
                        let even_size = Size::new(rect.size.width / children_count, rect.size.height);
                        child.calculate_bound_sizes(even_size)
                    }
                    LayoutDirection::Vertical => {
                        let even_size = Size::new(rect.size.width, rect.size.height / children_count);
                        child.calculate_bound_sizes(even_size)
                    }
                }
            };

            let child_rect = Rectangle::new(rect.top_left + offset, child_size);
            child.computed_rect = child_rect;
            child.layout(context, child_rect);

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

    fn draw(&mut self, context: &mut UiContext<'a, D, C>, _rect: Rectangle) {
        for child in &mut self.children {
            child.draw(context);
        }
    }
}
