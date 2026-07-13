#![no_std]
extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
pub use edgy_graphics as graphics;
use edgy_graphics::{framebuffer::FrameBuffer, geometry::{Point, Rectangle, Size}};
use slotmap::{SlotMap, new_key_type};

use crate::{context::{DrawContext, LayoutContext, SizeContext}, geometry::Constraint, widgets::{Widget, linear_layout::{LayoutAlignment, LayoutDirection, LinearLayout}, root::RootLayout}};

pub mod context;
pub mod geometry;
pub mod tree;
pub mod widgets;

pub type Tree = SlotMap<NodeId, Node>;

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

pub enum Prop<T> {
    Value(T),
    Binding(Box<dyn Fn() -> T>),
}

impl<T: Clone> Prop<T> {
    pub fn get(&self) -> T {
        match self {
            Prop::Value(value) => value.clone(),
            Prop::Binding(value) => value(),
        }
    }
}

new_key_type! {
    pub struct NodeId;
}

pub struct Node {
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
    size: Size,
    pub top_left: Point,
    /// Current widget object which belongs to this node. Option used for `.take()` pattern in mutable contexts
    pub widget: Option<Box<dyn Widget>>,
}

impl Node {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        Self {
            parent: None,
            first_child: None,
            next_sibling: None,
            widget: Some(widget),
            top_left: Point::<i32>::zero(),
            size: Size::zero()
        }
    }

    pub fn rectangle(&self) -> Rectangle {
        Rectangle::new(self.top_left, self.size)
    }
}

impl Into<Rectangle> for Node {
    fn into(self) -> Rectangle {
        self.rectangle()
    }
}

pub struct UiContext<M> {
    messages: heapless::Vec<M, 4>,
    pub tree: Tree,
    /// Ui context size
    pub viewport_size: Size,
    root: Option<NodeId>,
}


impl<M> UiContext<M> {
    pub fn new(viewport_size: Size) -> Self {
        Self {
            messages: heapless::Vec::new(),
            tree: SlotMap::with_key(),
            root: None,
            viewport_size,
        }
    }
    
    pub fn layout(&mut self) {
        if let Some(root) = self.root {
            self.tree
                .get_mut(root)
                .unwrap();
            self.measure_node(root, Constraint::loose(self.viewport_size));
            self.layout_node(root);
        }
    }

    pub fn draw(&mut self, framebuffer: &mut FrameBuffer) {
        if let Some(root) = self.root {
            self.draw_node(root, framebuffer);
        }
    }

    pub fn remove(&mut self, id: NodeId) {
        tree::remove(&mut self.tree, id);
    }

    pub fn resize(&mut self, size: Size) {
        self.viewport_size = size;
    
        let root = self.tree.get_mut(self.root.expect("root is not created yet")).unwrap();
        root.size = size;
    }
    
    fn add_widget_node(
        &mut self,
        widget: Box<dyn Widget>,
    ) -> NodeId {
        let node = Node::new(widget);
        self.tree.insert(node)
    }
    
    fn add_root_node(&mut self) -> NodeId {
        let node = Node::new(Box::new(RootLayout) as Box<dyn Widget>);
        self.tree.insert(node)
    }

    pub fn build(&mut self, f: impl FnOnce(&mut UiBuilder<M>)) {
        let root = self.add_root_node();
        self.root = Some(root);
        let mut builder = UiBuilder::new(self, root);
        f(&mut builder);
    }

    pub fn modify<T: Widget + 'static>(
        &mut self,
        id: NodeId,
        f: impl FnOnce(&mut T),
    ) {
        let node = self.tree.get_mut(id).unwrap();
        let widget = node.widget.as_mut().unwrap();
        let widget = widget.as_any_mut().downcast_mut::<T>().unwrap();
        f(widget);
    }

    fn layout_node(&mut self, id: NodeId) {
        let mut widget = self.tree.get_mut(id).unwrap().widget.take().expect("failed to take");
        {
            let mut cx = LayoutContext::new(&mut self.tree, id);
            widget.layout(&mut cx);
        }
        self.tree.get_mut(id).unwrap().widget = Some(widget);
    }

    fn measure_node(&mut self, id: NodeId, constraint: Constraint) {
        let mut widget = self.tree.get_mut(id).unwrap().widget.take().unwrap();
        {
            let mut cx = SizeContext::new(&mut self.tree, id);
            widget.measure(&mut cx, constraint);
        }
        self.tree.get_mut(id).unwrap().widget = Some(widget);
    }

    fn draw_node(&mut self, id: NodeId, framebuffer: &mut FrameBuffer) {
        {
            let node = self.tree.get_mut(id).unwrap();
            let widget = node.widget.as_mut().unwrap();

            let mut cx = DrawContext::new(Rectangle::new(node.top_left, node.size), framebuffer);

            widget.draw(&mut cx);
        }

        let mut child = self.tree.get(id).and_then(|n| n.first_child);

        while let Some(child_id) = child {
            let next = self.tree.get(child_id).and_then(|n| n.next_sibling);

            self.draw_node(child_id, framebuffer);
            child = next;
        }
    }
}

pub struct UiBuilder<'a, M> {
    ui: &'a mut UiContext<M>,
    parent: NodeId,
}

impl<'a, M> UiBuilder<'a, M> {
    fn new(ui: &'a mut UiContext<M>, parent: NodeId) -> Self {
        Self { ui, parent }
    }

    pub fn remove(&mut self, id: NodeId) {
        self.ui.remove(id);
    }

    pub fn add<W: Into<Box<dyn Widget>>>(&mut self, widget: W) -> NodeId {
        let id = self.ui.add_widget_node(widget.into());
        tree::attach_child(&mut self.ui.tree, self.parent, id);
        id
    }

    pub fn column(&mut self, f: impl FnOnce(&mut UiBuilder<M>)) -> NodeId {
        let id = self.add(LinearLayout::new(LayoutDirection::Horizontal, LayoutAlignment::Stretch, LayoutAlignment::Stretch, 0));
        let mut child_builder = UiBuilder::new(self.ui, id);
        f(&mut child_builder);
        id
    }
}