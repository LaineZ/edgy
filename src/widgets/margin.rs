use alloc::boxed::Box;
use embedded_graphics::{prelude::*, primitives::Rectangle};

use super::{UiBuilder, Widget, WidgetObj};

#[derive(Default, Debug, Copy, Clone)]
pub struct Margin {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

#[macro_export]
macro_rules! margin {
    ($all:expr) => {
        Margin {
            top: $all,
            right: $all,
            bottom: $all,
            left: $all,
        }
    };

    ($vertical:expr, $horizontal:expr) => {
        Margin {
            top: $vertical,
            right: $horizontal,
            bottom: $vertical,
            left: $horizontal,
        }
    };

    ($top:expr, $right:expr, $bottom:expr, $left:expr) => {
        Margin {
            top: $top,
            right: $right,
            bottom: $bottom,
            left: $left,
        }
    };
}

pub struct MarginLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub(crate) margin: Margin,
    pub(crate) child: Option<WidgetObj<'a, D, C>>,
}

impl<'a, D, C> UiBuilder<'a, D, C> for MarginLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObj<'a, D, C>) {
        if self.child.is_none() {
            self.child = Some(widget);
        } else {
            panic!("MarginContainer can only have one child!");
        }
    }

    fn finish(self) -> WidgetObj<'a, D, C> {
        if self.child.is_none() {
            panic!("MarginContainer must have a child before finishing!");
        }
        WidgetObj {
            widget: Box::new(self),
        }
    }
}

pub struct MarginLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub child: Option<WidgetObj<'a, D, C>>,
    pub margin: Margin,
}

impl<'a, D, C> UiBuilder<'a, D, C> for MarginLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObj<'a, D, C>) {
        self.child = Some(widget);
    }

    fn finish(self) -> WidgetObj<'a, D, C> {
        WidgetObj {
            widget: Box::new(MarginLayout {
                margin: self.margin,
                child: self.child,
            }),
        }
    }
}
impl<'a, D, C> Widget<'a, D, C> for MarginLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn size(&mut self, hint: Size) -> Size {
        let available_width = hint
            .width
            .saturating_sub((self.margin.left + self.margin.right) as u32);
        let available_height = hint
            .height
            .saturating_sub((self.margin.top + self.margin.bottom) as u32);
        let available_size = Size::new(available_width.max(0), available_height.max(0));

        let child_size = self.child.as_mut().unwrap().size(available_size);

        Size::new(
            child_size.width + (self.margin.left + self.margin.right) as u32,
            child_size.height + (self.margin.top + self.margin.bottom) as u32,
        )
    }

    fn draw(
        &mut self,
        context: &mut crate::UiContext<'a, D, C>,
        rect: embedded_graphics::primitives::Rectangle,
    ) {
        let available_width = rect
            .size
            .width
            .saturating_sub((self.margin.left + self.margin.right) as u32);
        let available_height = rect
            .size
            .height
            .saturating_sub((self.margin.top + self.margin.bottom) as u32);
        let available_size = Size::new(available_width, available_height);

        let child_size = self.child.as_mut().unwrap().size(available_size);

        let child_rect = Rectangle::new(
            Point::new(
                rect.top_left.x + self.margin.left,
                rect.top_left.y + self.margin.top,
            ),
            child_size,
        );

        self.child.as_mut().unwrap().draw(context, child_rect);
    }
}
