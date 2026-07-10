use edgy_graphics::{
    draw::{self, BasicStyle}, framebuffer::FrameBuffer, geometry::{Point, Rectangle, Size},
};

/// Margin struct
#[derive(Default, Debug, Copy, Clone)]
pub struct MarginSize {
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
    pub left: u16,
}

pub type Padding = MarginSize;

/// Macro that returns [Margin]. Defines in CSS fashion
/// `margin!(top, right, bottom, left)`
/// `margin!(vertical, horizontal)`
/// `margin!(all sides)`
#[macro_export]
macro_rules! margin {
    ($all:expr) => {
        $crate::decorators::MarginSize {
            top: $all,
            right: $all,
            bottom: $all,
            left: $all,
        }
    };

    ($vertical:expr, $horizontal:expr) => {
        $crate::decorators::MarginSize {
            top: $vertical,
            right: $horizontal,
            bottom: $vertical,
            left: $horizontal,
        }
    };

    ($top:expr, $right:expr, $bottom:expr, $left:expr) => {
        $crate::decorators::MarginSize {
            top: $top,
            right: $right,
            bottom: $bottom,
            left: $left,
        }
    };
}

use crate::widgets::View;

pub struct Background<V> {
    inner: V,
    style: BasicStyle,
}

impl<V> Background<V> {
    pub fn new(inner: V, style: BasicStyle) -> Self {
        Self { inner, style }
    }
}

impl<V: View> View for Background<V> {
    type State = V::State;

    fn measure(&mut self, hint: Size) -> Size {
        self.inner.measure(hint)
    }

    fn layout(&mut self, rect: Rectangle, state: &Self::State) -> Rectangle {
        self.inner.layout(rect, state)
    }

    fn draw(&mut self, fb: &mut FrameBuffer, rect: Rectangle, state: &Self::State) {
        draw::rect(fb, rect, self.style);
        self.inner.draw(fb, rect, state);
    }
}

pub struct Margin<V> {
    inner: V,
    margin: MarginSize,
}

impl<V> Margin<V> {
    pub fn new(inner: V, margin: MarginSize) -> Self {
        Self { inner, margin }
    }
}

impl<V: View> View for Margin<V> {
    type State = V::State;

    fn measure(&mut self, hint: Size) -> Size {
        let available_width = hint
            .width
            .saturating_sub((self.margin.left + self.margin.right) as u32);
        let available_height = hint
            .height
            .saturating_sub((self.margin.top + self.margin.bottom) as u32);
        let available_size = Size::new(available_width, available_height);

        let child_size = self.inner.measure(available_size);

        Size::new(
            child_size.width + (self.margin.left + self.margin.right) as u32,
            child_size.height + (self.margin.top + self.margin.bottom) as u32,
        )
    }

    fn layout(&mut self, rect: Rectangle, state: &Self::State) -> Rectangle {
        let margin_rect = Rectangle {
            top_left: Point::new(
                rect.top_left.x + self.margin.left as i32,
                rect.top_left.y + self.margin.top as i32,
            ),
            size: Size::new(
                rect.size
                    .width
                    .saturating_sub(self.margin.left as u32 + self.margin.right as u32),
                rect.size
                    .height
                    .saturating_sub(self.margin.top as u32 + self.margin.bottom as u32),
            ),
        };
    
        self.inner.layout(margin_rect, state);
        margin_rect
    }

    fn draw(&mut self, fb: &mut FrameBuffer, rect: Rectangle, state: &Self::State) {
        self.inner.draw(fb, rect, state);
    }
}


pub trait ViewExt: View + Sized {
    fn background(self, style: BasicStyle) -> Background<Self> {
        Background::new(self, style)
    }

    fn margin(self, margin: MarginSize) -> Margin<Self> {
        Margin::new(self, margin)
    }
}

impl<T: View> ViewExt for T {}