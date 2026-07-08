use edgy_graphics::{framebuffer::{self, FrameBuffer}, geometry::{Rectangle, Size}};

use crate::{Event, UiContext};

pub mod root_layout;
pub mod label;

pub trait Behavior {
    type State;

    fn handle(&mut self, event: Event);
    fn state(&self) -> Self::State;
}

pub trait View {
    type State;

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

    fn draw(
        &self,
        framebuffer: &mut FrameBuffer,
        rect: Rectangle,
        state: &Self::State
    );
}

pub struct WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{
    behavior: B,
    view: V,
}

impl<B, V> WidgetObject<B, V> where B: Behavior, V: View<State = B::State> {
    pub fn new(behavior: B, view: V) -> Self {
       Self {
           behavior, view
       } 
    }
    
    pub fn measure(&mut self, hint: Size) -> Size {
        self.view.measure(hint)
    }

    pub fn min_size(&mut self) -> Size {
        self.view.min_size()
    }

    pub fn max_size(&mut self) -> Size {
        self.view.min_size()
    }

    pub fn layout(&mut self, rect: Rectangle) {
        self.view.layout(rect);
    }

    pub fn draw(&self, fb: &mut FrameBuffer, rect: Rectangle) {
        self.view.draw(fb, rect, &self.behavior.state());
    }
}

/// No behavior for widget
pub struct NullBehavior;

impl Behavior for NullBehavior {
    type State = ();

    fn handle(&mut self, _event: Event) {}

    fn state(&self) -> Self::State {
        ()
    }
}