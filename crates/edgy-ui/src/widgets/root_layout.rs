use alloc::vec::Vec;
use edgy_graphics::geometry::{Rectangle, Size};

use crate::widgets::{Widget, WidgetObject};

#[derive(Clone, Copy, PartialEq)]
pub enum Anchor {
    TopLeft,
    Center,
}

struct WidgetAndPosition<'a, M>
{
    widget_object: WidgetObject<'a, M>,
    dimensions: Rectangle,
    anchor: Anchor,
}

pub struct RootLayout<'a, M> {
    children: Vec<WidgetAndPosition<'a, M>>
}

impl<'a, M> RootLayout<'a, M> {
    pub fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }

    pub fn add(&mut self, widget: WidgetObject<'a, M>, bounds: Rectangle, anchor: Anchor) {
        self.children.push(WidgetAndPosition { widget_object: widget, dimensions: bounds, anchor });
    }
}

impl<'a, M: 'a> Widget<'a, M> for RootLayout<'a, M> {
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

    fn draw(&mut self, ui: &mut crate::UiContext<M>, rect: Rectangle) {
        for child in self.children.iter_mut() {
            child.widget_object.draw(ui, rect);
        }
    }
} 