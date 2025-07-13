use alloc::boxed::Box;
use embedded_graphics::{
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use crate::{EventResult, UiContext};

use super::{UiBuilder, Widget, WidgetEvent, WidgetObject};


/// Margin struct
#[derive(Default, Debug, Copy, Clone)]
pub struct Margin {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

pub type Padding = Margin;

/// Macro that returns [Margin]. Defines in CSS fashion
/// `margin!(top, right, bottom, left)`
/// `margin!(vertical, horizontal)`
/// `margin!(all sides)`
#[macro_export]
macro_rules! margin {
    ($all:expr) => {
        $crate::widgets::margin_layout::Margin {
            top: $all,
            right: $all,
            bottom: $all,
            left: $all,
        }
    };

    ($vertical:expr, $horizontal:expr) => {
        $crate::widgets::margin_layout::Margin {
            top: $vertical,
            right: $horizontal,
            bottom: $vertical,
            left: $horizontal,
        }
    };

    ($top:expr, $right:expr, $bottom:expr, $left:expr) => {
        $crate::widgets::margin_layout::Margin {
            top: $top,
            right: $right,
            bottom: $bottom,
            left: $left,
        }
    };
}

/// Container with margins
pub struct MarginLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub(crate) margin: Margin,
    pub(crate) child: Option<WidgetObject<'a, D, C>>,
    pub(crate) style: PrimitiveStyle<C>,
}

impl<'a, D, C> MarginLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    pub fn new(margin: Margin) -> Self {
        Self {
            child: None,
            margin,
            style: PrimitiveStyle::default(),
        }
    }

    pub fn new_with_style(margin: Margin, style: PrimitiveStyle<C>) -> Self {
        Self {
            child: None,
            margin,
            style,
        }
    }
}

impl<'a, D, C> UiBuilder<'a, D, C> for MarginLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObject<'a, D, C>) {
        if self.child.is_none() {
            self.child = Some(widget);
        } else {
            panic!("MarginContainer already have a child!");
        }
    }

    fn finish(self) -> WidgetObject<'a, D, C> {
        if self.child.is_none() {
            panic!("MarginContainer must have a child before finishing!");
        }

        WidgetObject::new(Box::new(self))
    }
}

impl<'a, D, C> Widget<'a, D, C> for MarginLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut crate::UiContext<'a, D, C>, hint: Size) -> Size {
        let available_width = hint
            .width
            .saturating_sub((self.margin.left + self.margin.right) as u32);
        let available_height = hint
            .height
            .saturating_sub((self.margin.top + self.margin.bottom) as u32);
        let available_size = Size::new(available_width, available_height);

        let child_size = self.child.as_mut().unwrap().size(context, available_size);

        Size::new(
            child_size.width + (self.margin.left + self.margin.right) as u32,
            child_size.height + (self.margin.top + self.margin.bottom) as u32,
        )
    }

    fn layout(&mut self, context: &mut crate::UiContext<'a, D, C>, rect: Rectangle) {
        let available_width = rect
            .size
            .width
            .saturating_sub((self.margin.left + self.margin.right) as u32);
        let available_height = rect
            .size
            .height
            .saturating_sub((self.margin.top + self.margin.bottom) as u32);
        let available_size = Size::new(available_width, available_height);

        let child_size = self.child.as_mut().unwrap().size(context, available_size);

        let child_rect = Rectangle::new(
            Point::new(
                rect.top_left.x + self.margin.left,
                rect.top_left.y + self.margin.top,
            ),
            child_size,
        );

        self.child.as_mut().unwrap().layout(context, child_rect);
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        let _ = rect.into_styled(self.style).draw(&mut context.draw_target);
        self.child
            .as_mut()
            .unwrap()
            .draw(context, event_args.system_event)
    }
}
