#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use edgy_graphics::{framebuffer::FrameBuffer, geometry::Rectangle};
pub use edgy_graphics; 

use crate::widgets::{Behavior, View, WidgetObject, root_layout::{Anchor, RootLayout}};

pub mod widgets;

pub enum Event {
    
}

pub trait EdgyApplication {
    type State;
    type Message;

    fn ui(
        ui: &mut UiContext<Self::Message>,
        state: &Self::State,
    );

    fn update(
        state: &mut Self::State,
        msg: Self::Message,
    );
}

pub struct UiContext<M> {
    messages: Vec<M>,
    pub framebuffer: FrameBuffer,
}

impl<'a, M> UiContext<M> {
    pub fn new(framebuffer: FrameBuffer) -> Self {
        Self {
            messages: Vec::new(),
            framebuffer
        }
    }

    pub fn update<B, V>(&mut self, root: WidgetObject<B, V>) where B: Behavior, V: View<State = B::State>, {
        let fb_bounds = self.framebuffer.bounding_box();


        let mut root_layout = RootLayout::new();
        root_layout.add(root, fb_bounds, Anchor::TopLeft);
        let size = root_layout.measure(fb_bounds.size);
        let bounds = Rectangle::new(fb_bounds.top_left, size);
        root_layout.layout(bounds, &());
        root_layout.draw(&mut self.framebuffer, bounds, &());
    }
}