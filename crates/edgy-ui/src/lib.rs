#![no_std]

extern crate alloc;

use alloc::vec::Vec;
pub use edgy_graphics as graphics;
use edgy_graphics::{
    framebuffer::FrameBuffer,
    geometry::{Point, Rectangle},
};

use crate::widgets::{
    Behavior, View, WidgetObject,
    root_layout::{Anchor, RootLayout},
};

pub mod decorators;
pub mod widgets;

/// Filtered to specified widget event
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    /// Idle event (None, Null) event
    Idle,
    /// Focus event. E.g hover from mouse or widget cycler (tab)
    Focus,
    // Active press at surface. E.g touch or mouse click
    Active(Option<Point>),
    Drag(Point),
}

/// Your events that can be inserted into UI context
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SystemEvent {
    /// Idle event (None, Null) event
    Idle,
    /// Focus to specified widget ID
    FocusTo(usize),
    // Active selected specified widget ID,
    ActiveTo(usize),
    /// Active press at surface (e.g touch or mouse press)
    Active(Point),
    /// Movement at surface event (e.g mouse moved to element)
    Move(Point),
    /// Dragging at surface event (e.g mouse press and move)
    Drag(Point),
    /// Increase the value in specified step in range 0.0-1.0, used for sliders
    Increase(f32),
    /// Decreases the value in specified step in range 0.0-1.0, used for sliders
    Decrease(f32),
}

impl SystemEvent {
    fn is_motion_event(&self) -> bool {
        matches!(self, SystemEvent::FocusTo(_) | SystemEvent::Move(_))
    }
}

pub trait EdgyApplication {
    type State;
    type Message;

    fn ui(ui: &mut UiContext<Self::Message>, state: &Self::State);

    fn update(state: &mut Self::State, msg: Self::Message);
}

pub struct UiContext<M> {
    messages: Vec<M>,
    /// Event to pass in the library
    motion_event: SystemEvent,
    interaction_event: SystemEvent,
    pub framebuffer: FrameBuffer,
}

impl<'a, M> UiContext<M> {
    pub fn new(framebuffer: FrameBuffer) -> Self {
        Self {
            messages: Vec::new(),
            motion_event: SystemEvent::Idle,
            interaction_event: SystemEvent::Idle,
            framebuffer,
        }
    }

    pub fn push_event(&mut self, event: SystemEvent) {
        if event.is_motion_event() {
            self.motion_event = event;
        } else {
            self.interaction_event = event;
        }
    }

    pub fn update<B, V>(&mut self, root: WidgetObject<B, V>)
    where
        B: Behavior,
        V: View<State = B::State>,
    {
        let fb_bounds = self.framebuffer.bounding_box();
        let mut root_layout = RootLayout::new();
        root_layout.add(root, fb_bounds, Anchor::TopLeft);
        let size = root_layout.measure(fb_bounds.size);
        let bounds = Rectangle::new(fb_bounds.top_left, size);
        root_layout.layout(bounds, &());
        root_layout.draw(&mut self.framebuffer, bounds, &());
    }
}
