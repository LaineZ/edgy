use edgy_graphics::{framebuffer::{self, FrameBuffer}, geometry::{Rectangle, Size}};

use crate::{Event, UiContext};

pub mod root_layout;
pub mod label;

pub trait Widget {
    fn measure(&mut self, hint: Size) -> Size;

    fn layout(&mut self, rect: Rectangle);

    fn draw(
        &mut self,
        framebuffer: &mut FrameBuffer,
        rect: Rectangle,
    );

    fn handle(&mut self, event: Event);
}

pub trait Behavior {
    type State;

    fn handle(&mut self, event: Event);
    fn state(&self) -> &Self::State;
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
    fn layout(&mut self, _rect: Rectangle, state: &Self::State) {}

    fn draw(
        &mut self,
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
}


impl<B, V> Widget for WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{
    fn measure(&mut self, hint: Size) -> Size {
        self.view.measure(hint)
    }

    fn layout(&mut self, rect: Rectangle) {
        self.view.layout(rect, &self.behavior.state());
    }

    fn draw(
        &mut self,
        fb: &mut FrameBuffer,
        rect: Rectangle,
    ) {
        self.view.draw(
            fb,
            rect,
            &self.behavior.state(),
        );
    }

    fn handle(&mut self, event: Event) {
        self.behavior.handle(event);
    }
}

/// No behavior for widget
pub struct NullBehavior;

impl Behavior for NullBehavior {
    type State = ();

    fn handle(&mut self, _event: Event) {}

    fn state(&self) -> &Self::State {
        &()
    }
}