use alloc::boxed::Box;
use edgy_graphics::geometry::{Rectangle, Size};

use crate::UiContext;

pub mod root_layout;
pub mod label;

#[allow(unused_variables)]
pub trait Widget<'a, M>: 'a {
    /// Returns the size the widget wants. use for auto-calculate in layouts. Default implementation occupies all available space
    fn measure(&mut self, hint: Size) -> Size {
        hint
    }

    /// Returns a minimum size of widget
    fn min_size(&mut self) -> Size {
        Size::zero()
    }

    /// Returs a maximum size of widget
    fn max_size(&mut self) -> Size {
        Size::new(u32::MAX, u32::MAX)
    }

    /// Calls at layout pass. Gives a try for layout computation in Layouts (Containers)
    fn layout(&mut self, _rect: Rectangle) {}

    /// Widget drawing logic
    fn draw(&mut self, ui: &mut UiContext<M>, rect: Rectangle);
}


pub struct WidgetObject<'a, M> {
    pub widget: Box<dyn Widget<'a, M>>,
}

impl<'a, M: 'a> WidgetObject<'a, M> {
    pub fn new(widget: Box<dyn Widget<'a, M>>) -> Self {
       Self {
           widget
       } 
    }
    
    pub fn measure(&mut self, hint: Size) -> Size {
        self.widget.measure(hint)
    }

    pub fn min_size(&mut self) -> Size {
        self.widget.min_size()
    }

    pub fn max_size(&mut self) -> Size {
        self.widget.min_size()
    }

    pub fn layout(&mut self, rect: Rectangle) {
        self.widget.layout(rect);
    }

    pub fn draw(&mut self, ui: &mut crate::UiContext<M>, rect: Rectangle) {
        self.widget.draw(ui, rect);
    }
}