use edgy_graphics::geometry::{Point, Rectangle, Size};

use crate::widgets::Widget;

pub struct RootLayout {
    size: Size
}

impl RootLayout {
    pub fn new(size: Size) -> Self {
        Self {
            size
        }
    }
}

impl Widget for RootLayout {
    fn measure(&mut self, context: &mut crate::context::SizeContext<'_>, constraint: crate::geometry::Constraint) -> Size {
        let children = context.children();

        if children.len() > 1 {
            panic!("root layout contains more than 1 children");
        }
        
        for child in &children {
            context.measure_child(*child, constraint);
        }

        constraint.max_size
    }
    
    fn layout(&mut self, context: &mut crate::context::LayoutContext<'_>) {
        let constraint = context.constraint();
        let children = context.children();

        for &child in &children {
            context.set_child_constraint(child, constraint);
            context.layout_child(child);
        }
    }

    fn draw(&mut self, _context: &mut crate::context::DrawContext<'_>) {
    }
}