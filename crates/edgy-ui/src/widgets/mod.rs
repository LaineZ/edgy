use alloc::{boxed::Box, vec::Vec};
use edgy_graphics::{framebuffer::FrameBuffer, geometry::{Rectangle, Size}};

use crate::Event;

pub mod root_layout;
pub mod label;
pub mod linear_layout;

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
    fn layout(&mut self, _rect: Rectangle, _state: &Self::State) -> Rectangle {
        _rect
    }

    fn draw(
        &mut self,
        framebuffer: &mut FrameBuffer,
        rect: Rectangle,
        state: &Self::State
    );
}

pub trait Widget {
    fn measure(&mut self, hint: Size) -> Size;
    fn layout(&mut self, rect: Rectangle);
    fn draw(&mut self, fb: &mut FrameBuffer);
    fn handle(&mut self, event: Event);
    fn rect(&self) -> Rectangle;
    fn set_rect(&mut self, rect: Rectangle);
}

pub struct WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{
    behavior: B,
    view: V,
    computed_rect: Rectangle,
}

impl<B, V> WidgetObject<B, V> where B: Behavior, V: View<State = B::State> {
    pub fn new(behavior: B, view: V) -> Self {
       Self {
           behavior, view,
           computed_rect: Rectangle::zero(),
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
        let modified_rect = self.view.layout(rect, &self.behavior.state());
        self.set_rect(modified_rect);
    }

    fn draw(
        &mut self,
        fb: &mut FrameBuffer,
    ) {
        let state = self.behavior.state();
        self.view.draw(
            fb,
            self.rect(),
            &state,
        );
    }

    fn handle(&mut self, event: Event) {
        self.behavior.handle(event);
    }

    fn set_rect(&mut self, rect: Rectangle) {
        self.computed_rect = rect;
    }

    fn rect(&self) -> Rectangle {
        self.computed_rect
    }
}

impl<T: Widget + 'static> From<T> for Box<dyn Widget> {
    fn from(value: T) -> Self {
        Box::new(value)
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

pub struct Ui<'a> {
    children: &'a mut Vec<Box<dyn Widget>>,
}

impl<'a> Ui<'a> {
    pub fn add<W: Into<Box<dyn Widget>>>(&mut self, widget: W)
    {
        self.children.push(widget.into());
    }

    // TODO: Add more widgets here
}