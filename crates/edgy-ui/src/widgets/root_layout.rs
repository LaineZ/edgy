use alloc::{boxed::Box, vec::Vec};
use edgy_graphics::{
    framebuffer::FrameBuffer,
    geometry::{Rectangle, Size},
};

use crate::widgets::{Behavior, View, Widget, WidgetObject};

#[derive(Clone, Copy, PartialEq)]
pub enum Anchor {
    TopLeft,
    Center,
}

struct WidgetAndPosition<'a> {
    widget: Box<dyn Widget + 'a>,
    dimensions: Rectangle,
    anchor: Anchor,
}

pub struct RootLayout<'a> {
    children: Vec<WidgetAndPosition<'a>>,
}

impl<'a> RootLayout<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add<B, V>(&mut self, widget: WidgetObject<B, V>, bounds: Rectangle, anchor: Anchor)
    where
        B: Behavior + 'a,
        V: View<State = B::State> + 'a,
    {
        self.children.push(WidgetAndPosition {
            widget: Box::new(widget),
            dimensions: bounds,
            anchor,
        });
    }
}

impl<'a> View for RootLayout<'a> {
    type State = ();

    fn measure(&mut self, _hint: Size) -> Size {
        let mut size = Size::zero();

        for child in self.children.iter_mut() {
            let child_size = child.widget.measure(child.dimensions.size);
            size += child_size;
            if child.dimensions.size == Size::zero() {
                child.dimensions.size = child_size;
            }
        }

        size
    }

    fn layout(&mut self, rect: Rectangle, _state: &()) -> Rectangle {
        for child in self.children.iter_mut() {
            match child.anchor {
                Anchor::TopLeft => {
                    child.widget.layout(child.dimensions);
                }
                Anchor::Center => {
                    let centered_pos =
                        rect.top_left + (rect.size / 2) - (child.dimensions.size / 2);
                    let centered_rect = Rectangle::new(centered_pos, child.dimensions.size);
                    child.widget.layout(centered_rect);
                }
            }
        }

        rect
    }

    fn draw(&mut self, framebuffer: &mut FrameBuffer, _rect: Rectangle, _state: &()) {
        for child in self.children.iter_mut() {
            child.widget.draw(framebuffer);
        }
    }
}
