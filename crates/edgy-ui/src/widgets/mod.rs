use core::any::Any;

use alloc::boxed::Box;
use edgy_graphics::geometry::Size;

pub mod linear_layout;
pub mod label;
pub mod root;

use crate::{Event, EventResult, context::{DrawContext, LayoutContext, SizeContext}, geometry::Constraint};

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Widget: AsAny {
    fn measure(&mut self, context: &mut SizeContext<'_>, constraint: Constraint) -> Size {
        constraint.max_size
    }
    
    fn layout(&mut self, context: &mut LayoutContext<'_>) {}
    fn draw(&mut self, context: &mut DrawContext<'_>);
}

pub struct WidgetObject<B, V>
where
    B: Behavior,
    V: View<State = B::State>,
{
    behavior: B,
    view: V,
}

impl<B, V> Widget for WidgetObject<B, V>
where
    B: Behavior + 'static,
    V: View<State = B::State> + 'static,
{
    fn layout(&mut self, context: &mut LayoutContext<'_>) {
        self.view.layout(context);
    }

    fn draw(&mut self, context: &mut DrawContext) {
        self.view.draw(context);
    }
}

impl<T: Widget + 'static> From<T> for Box<dyn Widget> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

pub trait Behavior {
    type State: Default + 'static;
    fn handle(&mut self, state: &mut Self::State, event: Event) -> EventResult {
        EventResult::Pass
    }
}

pub trait View {
    type State;
    fn layout(&mut self, _context: &mut LayoutContext<'_>) {}
    fn draw(&mut self, _context: &mut DrawContext<'_>) {}
}