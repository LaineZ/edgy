use super::{UiBuilder, Widget, WidgetEvent, WidgetObject};
use crate::{style::{SelectorKind, Style}, EventResult, SystemEvent, UiContext};
use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Grid layout. Places items in the specified grid
pub struct GridLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub children: Vec<WidgetObject<'a, D, C>>,
    pub col_fracs: Vec<u32>,
    pub row_fracs: Vec<u32>,
    pub gap: u32,
}

impl<D, C> GridLayoutBuilder<'_, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Adds a column to the grid (specified in percents)
    pub fn add_column(mut self, percentage: u32) -> Self {
        self.col_fracs.push(percentage.clamp(0, 100));
        self
    }

    /// Adds a row to the grid (specified in percents)
    pub fn add_row(mut self, percentage: u32) -> Self {
        self.row_fracs.push(percentage.clamp(0, 100));
        self
    }

    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }
}

impl<D, C> Default for GridLayoutBuilder<'_, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            children: Vec::new(),
            col_fracs: Vec::new(),
            row_fracs: Vec::new(),
            gap: 0,
        }
    }
}

impl<'a, D, C> UiBuilder<'a, D, C> for GridLayoutBuilder<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn add_widget_obj(&mut self, widget: WidgetObject<'a, D, C>) {
        self.children.push(widget);
    }

    fn finish(self, selectors: &'a [SelectorKind]) -> WidgetObject<'a, D, C> {
        WidgetObject::new(Box::new(GridLayout {
            children: self.children,
            col_fracs: self.col_fracs,
            row_fracs: self.row_fracs,
            gap: self.gap,
        }), selectors)
    }
}

pub struct GridLayout<'a, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    pub children: Vec<WidgetObject<'a, D, C>>,
    pub col_fracs: Vec<u32>,
    pub row_fracs: Vec<u32>,
    pub gap: u32,
}

impl<'a, D, C> Widget<'a, D, C> for GridLayout<'a, D, C>
where
    D: DrawTarget<Color = C> + 'a,
    C: PixelColor + 'a,
{
    fn layout(&mut self, context: &mut UiContext<'a, D, C>, rect: Rectangle) {
        let cols = self.col_fracs.len();
        let rows = self.row_fracs.len();

        if cols == 0 || rows == 0 {
            panic!("column/row count must be greater than 0")
        }

        let total_gap_width = (cols.saturating_sub(1)) as u32 * self.gap;
        let total_gap_height = (rows.saturating_sub(1)) as u32 * self.gap;

        let available_width = rect.size.width.saturating_sub(total_gap_width);
        let available_height = rect.size.height.saturating_sub(total_gap_height);

        let total_col: u32 = self.col_fracs.iter().sum();
        let total_row: u32 = self.row_fracs.iter().sum();

        let mut col_widths: Vec<u32> = self
            .col_fracs
            .iter()
            .map(|&frac| available_width * frac / total_col)
            .collect();

        let mut row_heights: Vec<u32> = self
            .row_fracs
            .iter()
            .map(|&frac| available_height * frac / total_row)
            .collect();

        let total_actual_width: u32 = col_widths.iter().sum();
        if total_actual_width != available_width {
            col_widths[cols - 1] =
                col_widths[cols - 1].saturating_add(available_width - total_actual_width);
        }

        let total_actual_height: u32 = row_heights.iter().sum();
        if total_actual_height != available_height {
            row_heights[rows - 1] =
                row_heights[rows - 1].saturating_add(available_height - total_actual_height);
        }

        for r in 0..rows {
            for c in 0..cols {
                let cell_index = r * cols + c;
                if cell_index >= self.children.len() {
                    break;
                }

                let x_offset: i32 = col_widths[..c]
                    .iter()
                    .map(|w| *w as i32 + self.gap as i32)
                    .sum();

                let y_offset: i32 = row_heights[..r]
                    .iter()
                    .map(|h| *h as i32 + self.gap as i32)
                    .sum();

                let cell_rect = Rectangle::new(
                    rect.top_left + Point::new(x_offset, y_offset),
                    Size::new(col_widths[c], row_heights[r]),
                );

                self.children[cell_index].layout(context, cell_rect);
            }
        }
    }

    fn draw(
        &mut self,
        context: &mut UiContext<'a, D, C>,
        _rect: Rectangle,
        event_args: WidgetEvent, resolved_style: &Style<'a, C>,
    ) -> EventResult {
        let mut event_result = EventResult::Pass;

        for child in self.children.iter_mut() {
            if event_result == EventResult::Stop {
                event_result = child.draw(context, &SystemEvent::Idle);
            } else {
                event_result = child.draw(context, event_args.system_event);
            }
        }

        event_result
    }
}
