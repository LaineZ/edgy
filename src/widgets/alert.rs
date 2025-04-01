use alloc::{boxed::Box, string::String};
use embedded_graphics::{
    mono_font::ascii::FONT_4X6,
    prelude::{DrawTarget, PixelColor, Size},
    primitives::Rectangle,
    text::Alignment,
};

use crate::{margin, themes::WidgetStyle, EventResult, UiContext, MAX_SIZE};

use super::{
    linear_layout::{LayoutAlignment, LayoutDirection, LinearLayoutBuilder},
    UiBuilder, Widget, WidgetEvent, WidgetObject,
};

pub struct Alert<'a, C: PixelColor, D: DrawTarget<Color = C>> {
    layout: WidgetObject<'a, D, C>,
    max_size: Size,
}

impl<'a, D, C> Alert<'a, C, D>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    pub fn new(text: String, style: WidgetStyle<C>, mut callback: Box<dyn FnMut() + 'a>) -> Self {
        let mut layout = LinearLayoutBuilder::default()
            .direction(LayoutDirection::Vertical)
            .vertical_alignment(LayoutAlignment::Stretch)
            .horizontal_alignment(LayoutAlignment::Stretch)
            .style(style);

        layout.margin_layout(margin!(5), |ui| {
            ui.label(&text, Alignment::Left, &FONT_4X6);
        });

        layout.button("OK", &FONT_4X6, move || (callback)());

        Self {
            max_size: MAX_SIZE,
            layout: layout.finish(),
        }
    }
}

impl<'a, D, C> Widget<'a, D, C> for Alert<'a, C, D>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn size(&mut self, context: &mut UiContext<'a, D, C>, hint: Size) -> Size {
        self.max_size = context.draw_target.bounding_box().size;
        self.layout.size(context, hint)
    }

    fn max_size(&mut self) -> Size {
        self.max_size
    }

    fn layout(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        self.layout.layout(context, rect);
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        _rect: Rectangle,
        event_args: WidgetEvent,
    ) -> EventResult {
        context.dim_screen();
        self.layout.draw(context, event_args.system_event);
        EventResult::Stop
    }
}
