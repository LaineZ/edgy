use edgy_graphics::geometry::Rectangle;

use crate::{Event, EventDispatcher, EventResult, SystemEvent, WidgetId, widgets::Behavior};

pub(crate) fn handle_system_event<B: Behavior>(
    behavior: &mut B,
    rect: Rectangle,
    id: WidgetId,
    storage: &mut crate::StateStorage,
    dispatcher: &mut EventDispatcher,
) -> EventResult {
    let state = storage.get_or_insert::<B::State>(id);

    match dispatcher.current() {
        SystemEvent::PointerMove(point) if !dispatcher.pointer_down && rect.contains(point) => {
            behavior.handle(state, Event::HoverEnter)
        }

        SystemEvent::PointerDown(point) if rect.contains(point) => {
            behavior.handle(state, Event::Press)
        }

        SystemEvent::PointerUp(point) if rect.contains(point) => {
            behavior.handle(state, Event::Release)
        }

        _ => EventResult::Pass,
    }
}