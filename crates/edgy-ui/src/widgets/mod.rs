use core::marker::PhantomData;

use alloc::{boxed::Box, vec::Vec};
use edgy_graphics::{
    draw::{self, BasicStyle},
    framebuffer::FrameBuffer,
    geometry::{Rectangle, Size},
};

use crate::{Event, EventDispatcher, EventResult, SystemEvent, utils};

pub mod label;
pub mod linear_layout;
pub mod root_layout;

pub trait Behavior {
    type State;

    fn handle(&mut self, event: Event) -> EventResult;
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
    fn layout(&mut self, _rect: Rectangle, _state: &Self::State) {}

    fn draw(&mut self, framebuffer: &mut FrameBuffer, rect: Rectangle, state: &Self::State);
}

pub trait Widget {
    fn measure(&mut self, hint: Size) -> Size;
    fn layout(&mut self, rect: Rectangle);
    fn draw(&mut self, fb: &mut FrameBuffer);
    fn handle_system_event(&mut self, dispatcher: &EventDispatcher) -> EventResult;
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
    hovered: bool,
}

impl<B, V> WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{
    pub fn new(behavior: B, view: V) -> Self {
        Self {
            behavior,
            view,
            hovered: false,
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
        self.view.layout(rect, &self.behavior.state());
        self.set_rect(rect);
    }

    fn handle_system_event(&mut self, dispatcher: &EventDispatcher) -> EventResult {
        let rect = self.rect();
        match dispatcher.current() {
            SystemEvent::PointerMove(point) => {
                let inside = rect.contains(point);
    
                match (inside, self.hovered) {
                    (true, false) => {
                        self.hovered = true;
                        self.behavior.handle(Event::HoverEnter)
                    }
    
                    (false, true) => {
                        self.hovered = false;
                        self.behavior.handle(Event::HoverLeave)
                    }
    
                    _ => EventResult::Pass,
                }
            }
    
            SystemEvent::PointerDown(point) if rect.contains(point) => {
                self.behavior.handle(Event::Press)
            }
    
            SystemEvent::PointerUp(point) if rect.contains(point) => {
                self.behavior.handle(Event::Release)
            }
    
            _ => EventResult::Pass,
        }
    }

    fn draw(&mut self, fb: &mut FrameBuffer) {
        let state = self.behavior.state();
        self.view.draw(fb, self.rect(), &state);

        draw::rect(fb, self.computed_rect, BasicStyle::with_border(40, 1));
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

    fn handle(&mut self, _event: Event) -> EventResult {
        EventResult::Pass
    }

    fn state(&self) -> &Self::State {
        &()
    }
}


/// No view for widget, useful for containers
pub struct NullView;

impl View for NullView {
    type State = ();
    
    fn draw(&mut self, _: &mut FrameBuffer, _: Rectangle, _: &Self::State) {}
}

pub struct Ui<'a> {
    children: &'a mut Vec<Box<dyn Widget>>,
}

impl<'a> Ui<'a> {
    pub fn add<W: Into<Box<dyn Widget>>>(&mut self, widget: W) {
        self.children.push(widget.into());
    }

    // TODO: Add more widgets here
}
