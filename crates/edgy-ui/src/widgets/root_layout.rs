use alloc::{boxed::Box, vec::Vec};
use edgy_graphics::{
    framebuffer::FrameBuffer,
    geometry::{Rectangle, Size},
};

use crate::{EventDispatcher, EventResult, utils, widgets::{Behavior, View, Widget, WidgetObject}};

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
    computed_rect: Rectangle
}

impl<'a> RootLayout<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            computed_rect: Rectangle::zero()
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget + 'a>, bounds: Rectangle, anchor: Anchor)
    {
        self.children.push(WidgetAndPosition {
            widget: widget,
            dimensions: bounds,
            anchor,
        });
    }
}

impl<'a> Widget for RootLayout<'a> {
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

    fn layout(&mut self, rect: Rectangle) {
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
        self.set_rect(rect);
    }

    fn draw(&mut self, fb: &mut FrameBuffer) {
        for child in self.children.iter_mut() {
            child.widget.draw(fb);
        }
    }

    fn handle_system_event(&mut self, event: &EventDispatcher) -> crate::EventResult {
        for wd in self.children.iter_mut().rev() {
            let result = wd.widget.handle_system_event(event);

            if result == EventResult::Stop {
                return result;
            }
        }

        EventResult::Pass
    }

    fn rect(&self) -> Rectangle {
        self.computed_rect
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.computed_rect = rect;
    }
}
