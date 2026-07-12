use edgy_graphics::{geometry::{Point, Rectangle, Size}};

use crate::{
    context::LayoutContext,
    geometry::Constraint,
    widgets::Widget,
};

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

impl LayoutAlignment {
    fn offset(self, free: u32) -> u32 {
        match self {
            LayoutAlignment::Center => free / 2,
            LayoutAlignment::End => free,
            _ => 0,
        }
    }
}

pub struct LinearLayout {
    direction: LayoutDirection,
    horizontal_alignment: LayoutAlignment,
    vertical_alignment: LayoutAlignment,
    gap: u16,
}

impl LinearLayout {
    pub fn new(
        direction: LayoutDirection,
        horizontal_alignment: LayoutAlignment,
        vertical_alignment: LayoutAlignment,
        gap: u16,
    ) -> LinearLayout {
        Self {
            direction,
            horizontal_alignment,
            vertical_alignment,
            gap,
        }
    }
}

impl Widget for LinearLayout {
    fn measure(
        &mut self,
        context: &mut crate::context::SizeContext<'_>,
        constraint: Constraint,
    ) -> Size {
        let hint = constraint.max_size;
        let children = context.children();
        let mut computed_size = Size::zero();
        let gap_total = self.gap as u32 * children.len().saturating_sub(1) as u32;

        for child_id in children {
            // oh dear...
            let remaining_size = match self.direction {
                LayoutDirection::Horizontal => {
                    Size::new(hint.width.saturating_sub(computed_size.width), hint.height)
                }
                LayoutDirection::Vertical => {
                    Size::new(hint.width, hint.height.saturating_sub(computed_size.height))
                }
            };

            let child_size = context.measure_child(child_id, Constraint::loose(remaining_size));

            match self.direction {
                LayoutDirection::Horizontal => {
                    computed_size.width += child_size.width + gap_total;
                    computed_size.height = computed_size.height.max(child_size.height);
                }
                LayoutDirection::Vertical => {
                    computed_size.width = computed_size.width.max(child_size.width);
                    computed_size.height += child_size.height + gap_total;
                }
            }
        }

        if hint != Size::zero() {
            computed_size.min(hint)
        } else {
            computed_size
        }
    }

    fn layout(&mut self, context: &mut LayoutContext<'_>) {
        let children = context.children();
        let children_count = children.len();
        let rect = context.rect();
        let total_gap = self.gap as u32 * children.len().saturating_sub(1) as u32;
        let total_length = match self.direction {
            LayoutDirection::Horizontal => {
                let mut total = 0;
                for child in children.iter() {
                    let child_size = context.get_child_costraint(*child);
                    total += child_size.max_size.width;
                }
                total
            }
            LayoutDirection::Vertical => {
                let mut total = 0;
                for child in children.iter() {
                    let child_size = context.get_child_costraint(*child);
                    total += child_size.max_size.height;
                }
                total
            }
        } + total_gap;

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

        // compute stretched size
        let stretched_size = if main_alignment == LayoutAlignment::Stretch {
            match self.direction {
                LayoutDirection::Horizontal => rect.size.width / children_count as u32,
                LayoutDirection::Vertical => rect.size.height / children_count as u32,
            }
        } else {
            0 // just do not stretch
        };

        for (i, child) in children.iter().enumerate() {
            let child_bounds = Size::new(rect.size.width, rect.size.height);
            let mut child_size = context.get_child_size(*child);

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

            context.set_child_position(*child, child_rect.top_left);
            context.set_child_size(*child, child_rect.size);

            match self.direction {
                LayoutDirection::Horizontal => {
                    main_offset += child_size.width as i32;
                    if i != children_count - 1 {
                        main_offset += self.gap as i32;
                    }
                }
                LayoutDirection::Vertical => {
                    main_offset += child_size.height as i32;
                    if i != children_count - 1 {
                        main_offset += self.gap as i32;
                    }
                }
            }
        }
    }

    fn draw(&mut self, _context: &mut crate::context::DrawContext<'_>) {}
}
