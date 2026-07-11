#![no_std]
extern crate alloc;

use core::any::Any;

use alloc::{boxed::Box, collections::btree_map::BTreeMap, vec::Vec};
pub use edgy_graphics as graphics;
use edgy_graphics::{
    framebuffer::FrameBuffer,
    geometry::{Point, Rectangle},
};
use heapless::Deque;

use crate::widgets::{Behavior, Widget, root_layout::{Anchor, RootLayout}};

pub mod decorators;
pub mod widgets;
pub(crate) mod utils;

pub type WidgetId = u64;
const ROOT_ID: WidgetId = 0;

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
    pub(crate) pointer_down: bool,
    pub hovered_widget: Option<WidgetId>,
    pub(crate) old_hovered_widget: Option<WidgetId>,
}

impl EventDispatcher {
    pub fn update(&mut self, event: SystemEvent) {
        self.old_hovered_widget = self.hovered_widget;
        self.hovered_widget = None;
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


pub struct IdGenerator {
    stack: Vec<WidgetId>,
    child_indices: Vec<u32>,
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self {
            stack: alloc::vec![0xcbf29ce484222325], // FNV offset basis
            child_indices: alloc::vec![0],
        }
    }
}

fn hash(parent: WidgetId, child: u32) -> WidgetId {
    let mut h = parent ^ 0xcbf29ce484222325;

    for b in child.to_le_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }

    h
}

impl IdGenerator {
    pub fn next(&mut self) -> WidgetId {
        let parent = *self.stack.last().unwrap();
        let index = self.child_indices.last_mut().unwrap();
        let id = hash(parent, *index);
        *index += 1;
        id
    }

    pub fn push(&mut self, id: WidgetId) {
        self.stack.push(id);
        self.child_indices.push(0);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.child_indices.pop();
    }

    pub fn begin_frame(&mut self) {
        self.stack.clear();
        self.stack.push(ROOT_ID);

        self.child_indices.clear();
        self.child_indices.push(0);
    }
}

#[derive(Default)]
pub struct StateStorage {
    map: BTreeMap<WidgetId, Box<dyn Any>>,
}


impl StateStorage {
    pub fn get_or_insert<T>(
        &mut self,
        id: WidgetId,
    ) -> &mut T
    where
        T: Default + 'static,
    {
        self.map
            .entry(id)
            .or_insert_with(|| Box::new(T::default()))
            .downcast_mut::<T>()
            .expect("state type mismatch")
    }

    pub fn remove(&mut self, id: WidgetId) {
        self.map.remove(&id);
    }
}

pub struct UiContext<M> {
    messages: Vec<M>,
    events: Deque<SystemEvent, 32>,
    dispatcher: EventDispatcher,
    pub framebuffer: FrameBuffer,
    storage: StateStorage,
    id_generator: IdGenerator,
}

impl<M> UiContext<M> {
    pub fn new(framebuffer: FrameBuffer) -> Self {
        Self {
            messages: Vec::new(),
            framebuffer,
            id_generator: IdGenerator::default(),
            storage: StateStorage::default(),
            dispatcher: EventDispatcher::default(),
            events: Deque::new(),
        }
    }

    pub fn push_event(&mut self, event: SystemEvent) {
        self.events.push_back(event).ok();
    }

    pub fn update(&mut self, root: Box<dyn Widget>) {
        let fb_bounds = self.framebuffer.bounding_box();
        self.id_generator.begin_frame();

        let mut root_layout = RootLayout::new();
        root_layout.add(root, fb_bounds, Anchor::TopLeft);
        root_layout.init(&mut self.id_generator, &mut self.storage);
        let size = root_layout.measure(fb_bounds.size);
        let bounds = Rectangle::new(fb_bounds.top_left, size);
    
        root_layout.layout(bounds, &mut self.storage);
    
        while let Some(event) = self.events.pop_front() {
            self.dispatcher.update(event);
            root_layout.handle_system_event(&mut self.storage, &mut self.dispatcher);
        }
    
        root_layout.draw(&mut self.framebuffer, &mut self.storage);
    }
}
