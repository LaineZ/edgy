use alloc::{format, rc::Rc};
use core::cell::RefCell;
use embedded_graphics::{
    mono_font::ascii::FONT_4X6,
    prelude::{DrawTarget, PixelColor},
    text::Alignment,
};

use crate::DebugOptions;

use super::{
    linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder},
    UiBuilder, WidgetObject,
};

pub fn debug_options_ui<'a, D, C>(
    options_rc: Rc<RefCell<DebugOptions>>,
    select_id: usize,
) -> WidgetObject<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    let mut layout = LinearLayoutBuilder::default()
        .direction(LayoutDirection::Vertical)
        .vertical_alignment(LayoutAlignment::Start)
        .horizontal_alignment(LayoutAlignment::Stretch);

    // RUST - ЭТО ПИЗДЕЦ © thedrzj. я пероедаю rc потому что эта залупа заебала уже со своими лайфтмаймами

    layout.label(
        format!("selected widget: {}", select_id),
        Alignment::Left,
        &FONT_4X6,
    );
    layout.label("widget display", Alignment::Left, &FONT_4X6);
    layout.toggle_button("rects", &FONT_4X6, options_rc.borrow().widget_rects, {
        let options = options_rc.clone();
        move |state| {
            options.borrow_mut().widget_rects = state;
        }
    });

    layout.toggle_button(
        "active rects",
        &FONT_4X6,
        options_rc.borrow().widget_rect_active,
        {
            let options = options_rc.clone();
            move |state| {
                options.borrow_mut().widget_rect_active = state;
            }
        },
    );

    layout.toggle_button("sizes", &FONT_4X6, options_rc.borrow().widget_sizes, {
        let options = options_rc.clone();
        move |state| {
            options.borrow_mut().widget_sizes = state;
        }
    });

    layout.toggle_button("ids", &FONT_4X6, options_rc.borrow().widget_ids, {
        let options = options_rc.clone();
        move |state| {
            options.borrow_mut().widget_ids = state;
        }
    });

    layout.finish()
}
