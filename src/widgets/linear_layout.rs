use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{
    prelude::*,
    primitives::Rectangle,
};

use crate::{
    themes::WidgetStyle,
    EventResult, SystemEvent, UiContext,
};

use super::{UiBuilder, Widget, WidgetEvent, WidgetObject};

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

/// Builder for linear layout
pub struct LinearLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub children: Vec<WidgetObject<'a, D, C>>,
    pub horizontal_alignment: LayoutAlignment,
    pub vertical_alignment: LayoutAlignment,
    pub direction: LayoutDirection,
    pub style: WidgetStyle<C>,
    pub min_size: Size,
    pub max_size: Size,
}

impl<D, C> LinearLayoutBuilder<'_, D, C>
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

    pub fn horizontal_alignment(mut self, alignment: LayoutAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    pub fn alignment(mut self, alignment: LayoutAlignment) -> Self {
        if alignment == LayoutAlignment::Stretch {
            self.horizontal_alignment = LayoutAlignment::Stretch;
            self.vertical_alignment = LayoutAlignment::Stretch;
            return self;
        }

        match self.direction {
            LayoutDirection::Horizontal => {
                self.horizontal_alignment = alignment;
                self.vertical_alignment = LayoutAlignment::Start;
            }
            LayoutDirection::Vertical => {
                self.vertical_alignment = alignment;
                self.horizontal_alignment = LayoutAlignment::Start;
            }
        }
        self
    }

    pub fn style(mut self, style: WidgetStyle<C>) -> Self {
        self.style = style;
        self
    }

    pub fn vertical_alignment(mut self, alignment: LayoutAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    pub fn direction(mut self, direction: LayoutDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<D, C> Default for LinearLayoutBuilder<'_, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            children: Vec::new(),
            horizontal_alignment: LayoutAlignment::Start,
            vertical_alignment: LayoutAlignment::Start,
            style: WidgetStyle::default(),
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
    fn add_widget_obj(&mut self, widget: WidgetObject<'a, D, C>) {
        self.children.push(widget);
    }

    fn finish(self) -> WidgetObject<'a, D, C> {
        WidgetObject::new(Box::new(LinearLayout {
            direction: self.direction,
            children: self.children,
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
            style: self.style,
            min_size: self.min_size,
            max_size: self.max_size,
        }))
    }
}

/// Linear layout
pub struct LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    children: Vec<WidgetObject<'a, D, C>>,
    direction: LayoutDirection,
    horizontal_alignment: LayoutAlignment,
    vertical_alignment: LayoutAlignment,
    style: WidgetStyle<C>,
    min_size: Size,
    max_size: Size,
}

impl<'a, D, C> Widget<'a, D, C> for LinearLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        let mut computed_size = Size::zero();

        for child in &mut self.children {
            // oh dear...
            let remaining_size = match self.direction {
                LayoutDirection::Horizontal => {
                    Size::new(hint.width.saturating_sub(computed_size.width), hint.height)
                }
                LayoutDirection::Vertical => {
                    Size::new(hint.width, hint.height.saturating_sub(computed_size.height))
                }
            };

            let child_size = child.size(context, remaining_size);

            match self.direction {
                LayoutDirection::Horizontal => {
                    computed_size.width += child_size.width;
                    computed_size.height = computed_size.height.max(child_size.height);
                }
                LayoutDirection::Vertical => {
                    computed_size.width = computed_size.width.max(child_size.width);
                    computed_size.height += child_size.height;
                }
            }
        }

        if hint != Size::zero() {
            computed_size.min(hint)
        } else {
            computed_size
        }
    }

    fn max_size(&mut self) -> Size {
        self.max_size
    }

    fn min_size(&mut self) -> Size {
        self.min_size
    }

    fn layout(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let total_length = match self.direction {
            LayoutDirection::Horizontal => {
                let mut total = 0;
                for child in &mut self.children {
                    let child_size =
                        child.size(context, Size::new(rect.size.width, rect.size.height));
                    total += child_size.width;
                }
                total
            }
            LayoutDirection::Vertical => {
                let mut total = 0;
                for child in &mut self.children {
                    let child_size =
                        child.size(context, Size::new(rect.size.width, rect.size.height));
                    total += child_size.height;
                }
                total
            }
        };

        let main_axis_free_space = match self.direction {
            LayoutDirection::Horizontal => rect.size.width.saturating_sub(total_length),
            LayoutDirection::Vertical => rect.size.height.saturating_sub(total_length),
        };

        let main_alignment = if self.direction == LayoutDirection::Horizontal {
            self.horizontal_alignment
        } else {
            self.vertical_alignment
        };

        let mut main_offset = match main_alignment {
            LayoutAlignment::Center => main_axis_free_space / 2,
            LayoutAlignment::End => main_axis_free_space,
            _ => 0,
        } as i32;

        let children_count = self.children.len();

        // compute stretched size
        let stretched_size = if main_alignment == LayoutAlignment::Stretch {
            match self.direction {
                LayoutDirection::Horizontal => rect.size.width / children_count as u32,
                LayoutDirection::Vertical => rect.size.height / children_count as u32,
            }
        } else {
            0 // just do not stretch
        };

        for child in &mut self.children {
            let child_bounds = Size::new(rect.size.width, rect.size.height);
            let mut child_size = child.size(context, child_bounds);

            let cross_alignment = if self.direction == LayoutDirection::Horizontal {
                self.vertical_alignment
            } else {
                self.horizontal_alignment
            };

            match self.direction {
                LayoutDirection::Horizontal => {
                    if cross_alignment == LayoutAlignment::Stretch {
                        child_size.height = rect.size.height;
                    }

                    if main_alignment == LayoutAlignment::Stretch {
                        child_size.width = stretched_size;
                    }
                }
                LayoutDirection::Vertical => {
                    if cross_alignment == LayoutAlignment::Stretch {
                        child_size.width = rect.size.width;
                    }

                    if main_alignment == LayoutAlignment::Stretch {
                        child_size.height = stretched_size;
                    }
                }
            }

            let cross_offset = match self.direction {
                LayoutDirection::Horizontal => {
                    let free_space = rect.size.height.saturating_sub(child_size.height);
                    match self.vertical_alignment {
                        LayoutAlignment::Center => free_space / 2,
                        LayoutAlignment::End => free_space,
                        _ => 0,
                    }
                }
                LayoutDirection::Vertical => {
                    let free_space = rect.size.width.saturating_sub(child_size.width);
                    match self.horizontal_alignment {
                        LayoutAlignment::Center => free_space / 2,
                        LayoutAlignment::End => free_space,
                        _ => 0,
                    }
                }
            } as i32;

            let child_rect = match self.direction {
                LayoutDirection::Horizontal => Rectangle::new(
                    Point::new(
                        rect.top_left.x + main_offset,
                        rect.top_left.y + cross_offset,
                    ),
                    child_size,
                ),
                LayoutDirection::Vertical => Rectangle::new(
                    Point::new(
                        rect.top_left.x + cross_offset,
                        rect.top_left.y + main_offset,
                    ),
                    child_size,
                ),
            };

            child.computed_rect = child_rect;
            child.layout(context, child_rect);

            match self.direction {
                LayoutDirection::Horizontal => main_offset += child_size.width as i32,
                LayoutDirection::Vertical => main_offset += child_size.height as i32,
            }
        }
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        let _ = rect
            .into_styled(self.style.into())
            .draw(&mut context.draw_target);

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
