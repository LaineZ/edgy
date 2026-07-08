use alloc::vec::Vec;
use edgy_graphics::{framebuffer::FrameBuffer, geometry::{Rectangle, Size}};

use crate::widgets::{Behavior, View, WidgetObject};

#[derive(Clone, Copy, PartialEq)]
pub enum Anchor {
    TopLeft,
    Center,
}

struct WidgetAndPosition<B: Behavior, V: View<State = B::State>>
{
    widget_object: WidgetObject<B, V>,
    dimensions: Rectangle,
    anchor: Anchor,
}

pub struct RootLayout<B: Behavior, V: View<State = B::State>> {
    children: Vec<WidgetAndPosition<B, V>>
}

impl<B, V> RootLayout<B, V> where B: Behavior, V: View<State = B::State> {
    pub fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }

    pub fn add(&mut self, widget: WidgetObject<B, V>, bounds: Rectangle, anchor: Anchor) {
        self.children.push(WidgetAndPosition { widget_object: widget, dimensions: bounds, anchor });
    }
}

impl<B, V> View for RootLayout<B, V> where B: Behavior, V: View<State = B::State>  {
    type State = ();
    
    fn measure(&mut self, _hint: Size) -> Size {
        let mut size = Size::zero();

        for child in self.children.iter_mut() {
            let child_size = child.widget_object.measure(child.dimensions.size);
            size += child_size;
            if child.dimensions.size == Size::zero() {
                child.dimensions.size = child_size;
            }
        }

        size
    }

    fn layout(&mut self, rect: Rectangle) { 
        for child in self.children.iter_mut() {
            match child.anchor {
                Anchor::TopLeft => {
                    child.widget_object.layout(child.dimensions);
                }
                Anchor::Center => {
                    let centered_pos = rect.top_left
                        + (rect.size / 2)
                        - (child.dimensions.size / 2);
                    let centered_rect = Rectangle::new(centered_pos, child.dimensions.size);
                    child.widget_object.layout(centered_rect);
                }
            }
        }
    }

    fn draw(&self, framebuffer: &mut FrameBuffer, rect: Rectangle, _state: &()) {
        for child in self.children.iter() {
            child.widget_object.draw(framebuffer, rect);
        }
    }
} 