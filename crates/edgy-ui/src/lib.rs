#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use edgy_graphics::{framebuffer::FrameBuffer};
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
        let bounds = self.framebuffer.bounding_box();


        let mut root_layout = RootLayout::new();
        root_layout.add(root, bounds, Anchor::TopLeft);
        root_layout.measure(bounds.size);
        root_layout.layout(bounds);
        root_layout.draw(&mut self.framebuffer, bounds, &());
    }
}