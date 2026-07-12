use edgy_graphics::{framebuffer::FrameBuffer, geometry::{Point, Rectangle, Size}};
use slotmap::SlotMap;

use crate::{Node, NodeId, geometry::Constraint, tree};

pub struct SizeContext<'a> {
    pub(crate) tree: &'a mut SlotMap<NodeId, Node>,
    pub(crate) current: NodeId,
}

impl<'a> SizeContext<'a> {
    pub fn new(tree: &'a mut SlotMap<NodeId, Node>, current: NodeId) -> Self {
        Self { tree, current }
    }

    /// Returns ID of the current node
    pub fn id(&self) -> NodeId {
        self.current
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.tree.get(self.current).and_then(|n| n.parent)
    }

    pub fn children(&self) -> alloc::vec::Vec<NodeId> {
        tree::children(self.tree, self.current)
    }

    pub fn measure_child(&mut self, child_id: NodeId, constraint: Constraint) -> Size {
        let mut widget = {
            let node = self.tree.get_mut(child_id).unwrap();
            node.widget.take().unwrap()
        };
    
        let size_widget = {
            let mut child_cx = SizeContext::new(self.tree, child_id);
            widget.measure(&mut child_cx, constraint)
        };
    
        {
            let node = self.tree.get_mut(child_id).unwrap();
            node.size = size_widget;
            node.widget = Some(widget);
        }
    
        size_widget
    }
    
}

/// Widget layout context
pub struct LayoutContext<'a> {
    pub(crate) tree: &'a mut SlotMap<NodeId, Node>,
    pub(crate) current: NodeId,
}

impl<'a> LayoutContext<'a> {
    pub fn new(tree: &'a mut SlotMap<NodeId, Node>, current: NodeId) -> Self {
        Self { tree, current }
    }

    /// Returns ID of the current node
    pub fn id(&self) -> NodeId {
        self.current
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.tree.get(self.current).and_then(|n| n.parent)
    }

    pub fn set_child_size(&mut self, child: NodeId, size: Size) {
        self.tree.get_mut(child).unwrap().size = size;
    }

    pub fn get_child_size(&self, child: NodeId) -> Size {
        self.tree.get(child).unwrap().size
    }
    
    pub fn layout_child(&mut self, child_id: NodeId) {
        let mut widget = self.tree.get_mut(child_id).unwrap().widget.take().unwrap();
        {
            let mut child_cx = LayoutContext::new(self.tree, child_id);
            widget.layout(&mut child_cx);
        }
        self.tree.get_mut(child_id).unwrap().widget = Some(widget);
    }

    pub fn rect(&self) -> Rectangle {
        self.tree[self.id()].rectangle()
    }

    pub fn size(&self) -> Size {
        self.tree[self.id()].size
    }

    pub fn children(&self) -> alloc::vec::Vec<NodeId> {
        tree::children(self.tree, self.current)
    }

    pub fn children_of(&self, key: NodeId) -> alloc::vec::Vec<NodeId> {
        tree::children(self.tree, key)
    }

    pub fn set_child_position(&mut self, id: NodeId, position: Point) {
        self.tree.get_mut(id).unwrap().top_left = position;
    }

    pub fn child_rect(&self, key: NodeId) -> Rectangle {
        let node = &self.tree[key];
        Rectangle::new(node.top_left, node.size)
    }
}

/// Draw context
pub struct DrawContext<'a> {
    rectangle: Rectangle,
    pub framebuffer: &'a mut FrameBuffer,
}

impl<'a> DrawContext<'a> {
    pub fn new(rectangle: Rectangle, framebuffer: &'a mut FrameBuffer) -> Self {
        Self {
            rectangle,
            framebuffer,
        }
    }

    /// Returns current draw bounds for the widget
    pub fn rect(&self) -> Rectangle {
        self.rectangle
    }
}
