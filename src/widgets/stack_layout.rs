use embedded_graphics::{prelude::{DrawTarget, PixelColor, Size}, primitives::Rectangle};

use crate::UiContext;

use super::Widget;

pub struct StackLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    direction: StackLayoutDirection,
    widgets: Vec<Box<dyn Widget<D, C> + 'a>>,
}

impl<'a, D, C> StackLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub fn new<F>(direction: StackLayoutDirection, mut f: F) -> Self
    where
        F: FnMut(&mut dyn FnMut(Box<dyn Widget<D, C> + 'a>)),
    {
        let mut widgets = Vec::new();

        let mut add_widget = |widget: Box<dyn Widget<D, C> + 'a>| {
            widgets.push(widget);
        };

        f(&mut add_widget);

        Self { direction, widgets }
    }

    pub fn draw(&mut self, ui: &mut UiContext<D, C>) {
        let mut current_position = ui.bounds.top_left;
        let mut remaining_size = ui.bounds.size;

        for widget in &mut self.widgets {
            let widget_size = widget.size();

            if (self.direction == StackLayoutDirection::Vertical
                && widget_size.height > remaining_size.height)
                || (self.direction == StackLayoutDirection::Horizontal
                    && widget_size.width > remaining_size.width)
            {
                continue;
            }

            let mut child_ui = UiContext {
                draw_target: ui.draw_target,
                bounds: Rectangle::new(current_position, widget_size),
            };

            widget.draw(&mut child_ui);

            match self.direction {
                StackLayoutDirection::Vertical => {
                    current_position.y += widget_size.height as i32;
                    remaining_size.height -= widget_size.height;
                }
                StackLayoutDirection::Horizontal => {
                    current_position.x += widget_size.width as i32;
                    remaining_size.width -= widget_size.width;
                }
            }
        }
    }
}

impl<'a, D, C> Widget<D, C> for StackLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn draw(&mut self, ui: &mut UiContext<D, C>) {
        self.draw(ui)
    }

    fn size(&self) -> Size {
        let mut total_width = 0;
        let mut total_height = 0;

        for widget in &self.widgets {
            let size = widget.size();

            match self.direction {
                StackLayoutDirection::Vertical => {
                    total_width = total_width.max(size.width);
                    total_height += size.height;
                }
                StackLayoutDirection::Horizontal => {
                    total_width += size.width;
                    total_height = total_height.max(size.height);
                }
            }
        }

        Size::new(total_width, total_height)
    }
}

#[derive(PartialEq)]
pub enum StackLayoutDirection {
    Vertical,
    Horizontal,
}
