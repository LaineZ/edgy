use edgy_graphics::geometry::{Point, Rectangle, Size};

use crate::widgets::Widget;

pub struct RootLayout;

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
        for child in context.children() {
            context.set_child_position(child, Point::<i32>::zero());
            context.set_child_size(child, context.size());
            context.layout_child(child);
        }
    }

    fn draw(&mut self, _context: &mut crate::context::DrawContext<'_>) {
    }
}