#![no_std]
extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
pub use edgy_graphics as graphics;
use edgy_graphics::{
    framebuffer::FrameBuffer,
    geometry::{Point, Rectangle},
};
use heapless::Deque;

use crate::widgets::{Widget, root_layout::{Anchor, RootLayout}};

pub mod decorators;
pub mod widgets;
pub(crate) mod utils;

/// Event result struct
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventResult {
    /// Event processed
    Stop,
    /// Event passed, trying next widget
    Pass,
}


/// Filtered to specified widget event
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Event {
    /// Idle event (None, Null) event
    Idle,
    HoverEnter,
    HoverLeave,
    Press,
    Release,
}

/// Your events that can be inserted into UI context
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SystemEvent {
    /// Idle event (None, Null) event
    #[default]
    Idle,
    PointerMove(Point),
    PointerDown(Point),
    PointerUp(Point),
    NavigateNext,
    NavigatePrevious,
    Activate,
}

#[derive(Debug, Default)]
pub struct EventDispatcher {
    current: SystemEvent,
    pointer: Option<Point>,
    pointer_down: bool,
}

impl EventDispatcher {
    pub fn update(&mut self, event: SystemEvent) {
        self.current = event;

        match event {
            SystemEvent::PointerMove(pos) => {
                self.pointer = Some(pos);
            }

            SystemEvent::PointerDown(pos) => {
                self.pointer = Some(pos);
                self.pointer_down = true;
            }

            SystemEvent::PointerUp(pos) => {
                self.pointer = Some(pos);
                self.pointer_down = false;
            }

            _ => {}
        }
    }

    pub fn current(&self) -> SystemEvent {
        self.current
    }

    pub fn hit(&self, rect: Rectangle) -> bool {
        self.pointer.is_some_and(|p| rect.contains(p))
    }
}

pub struct UiContext<M> {
    messages: Vec<M>,
    events: Deque<SystemEvent, 32>,
    dispatcher: EventDispatcher,
    pub framebuffer: FrameBuffer,
}

impl<'a, M> UiContext<M> {
    pub fn new(framebuffer: FrameBuffer) -> Self {
        Self {
            messages: Vec::new(),
            framebuffer,
            dispatcher: EventDispatcher::default(),
            events: Deque::new()
        }
    }

    pub fn push_event(&mut self, event: SystemEvent) {
        self.events.push_back(event);
    }

    pub fn update(&mut self, root: Box<dyn Widget>) {
        let fb_bounds = self.framebuffer.bounding_box();
    
        let mut root_layout = RootLayout::new();
        root_layout.add(root, fb_bounds, Anchor::TopLeft);
    
        let size = root_layout.measure(fb_bounds.size);
        let bounds = Rectangle::new(fb_bounds.top_left, size);
    
        root_layout.layout(bounds);
    
        while let Some(event) = self.events.pop_front() {
            self.dispatcher.update(event);
            root_layout.handle_system_event(&self.dispatcher);
        }
    
        root_layout.draw(&mut self.framebuffer);
    }
}
