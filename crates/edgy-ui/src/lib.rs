#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use edgy_graphics::{framebuffer::FrameBuffer};
pub use edgy_graphics; 

use crate::widgets::{Widget, WidgetObject, root_layout::{Anchor, RootLayout}};

pub mod widgets;

pub type NodeId = usize;

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

impl<'a, M: 'a> UiContext<M> {
    pub fn new(framebuffer: FrameBuffer) -> Self {
        Self {
            messages: Vec::new(),
            framebuffer
        }
    }

    pub fn update(&mut self, root: WidgetObject<'a, M>) {
        let bounds = self.framebuffer.bounding_box();


        let mut root_layout = RootLayout::new();
        root_layout.add(root, bounds, Anchor::TopLeft);
        root_layout.measure(bounds.size);
        root_layout.layout(bounds);
        root_layout.draw(self, bounds);
    }
}