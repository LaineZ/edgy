use core::{alloc::Layout, marker::PhantomData};

use alloc::{boxed::Box, vec::Vec};
use edgy_graphics::{framebuffer::FrameBuffer, geometry::{Point, Rectangle, Size}};

use crate::widgets::{Ui, View, Widget, WidgetObject};

#[derive(PartialEq, Clone, Copy)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LayoutAlignment {
    Start,
    Center,
    End,
    Stretch,
}

/// Linear layout
pub struct LinearLayout<S>
{
    children: Vec<Box<dyn Widget>>,
    direction: LayoutDirection,
    horizontal_alignment: LayoutAlignment,
    vertical_alignment: LayoutAlignment,
    gap: u32,
    _state: PhantomData<S>
}

impl<S> LinearLayout<S> {
    pub fn new_vertical<F>(horizontal_alignment: LayoutAlignment, vertical_alignment: LayoutAlignment, gap: u32, build: F) -> Self
    where
        F: FnOnce(&mut Ui),
    {
        let mut layout = Self {
            children: Vec::new(),
            direction: LayoutDirection::Vertical,
            _state: PhantomData::<S>::default(),
            gap, horizontal_alignment, vertical_alignment
        };

        {
            let mut ui = Ui {
                children: &mut layout.children,
            };

            build(&mut ui);
        }

        layout
    }
}


impl<'a, S> View for LinearLayout<S> {
    type State = S;
    
    fn measure(&mut self, hint: Size) -> Size {
        let mut computed_size = Size::zero();
        let gap_total = self.gap * self.children.len().saturating_sub(1) as u32;

        for child in &mut self.children {
            // oh dear...
            let remaining_size = match self.direction {
                LayoutDirection::Horizontal => {
                    Size::new(hint.width.saturating_sub(computed_size.width), hint.height)
                }
                LayoutDirection::Vertical => {
                    Size::new(hint.width, hint.height.saturating_sub(computed_size.height))
                }
            };

            let child_size = child.measure(remaining_size);

            match self.direction {
                LayoutDirection::Horizontal => {
                    computed_size.width += child_size.width + gap_total;
                    computed_size.height = computed_size.height.max(child_size.height);
                }
                LayoutDirection::Vertical => {
                    computed_size.width = computed_size.width.max(child_size.width);
                    computed_size.height += child_size.height + gap_total;
                }
            }
        }

        if hint != Size::zero() {
            computed_size.min(hint)
        } else {
            computed_size
        }
    }

    fn layout(&mut self, rect: edgy_graphics::geometry::Rectangle, state: &Self::State) {
        let total_gap = self.gap * self.children.len().saturating_sub(1) as u32;
              let total_length = match self.direction {
                  LayoutDirection::Horizontal => {
                      let mut total = 0;
                      for child in &mut self.children {
                          let child_size =child.measure(Size::new(rect.size.width, rect.size.height));
                          total += child_size.width;
                      }
                      total
                  }
                  LayoutDirection::Vertical => {
                      let mut total = 0;
                      for child in &mut self.children {
                          let child_size = child.measure(Size::new(rect.size.width, rect.size.height));
                          total += child_size.height;
                      }
                      total
                  }
              } + total_gap;
      
              let main_axis_free_space = match self.direction {
                  LayoutDirection::Horizontal => rect.size.width.saturating_sub(total_length),
                  LayoutDirection::Vertical => rect.size.height.saturating_sub(total_length),
              };
      
              let main_alignment = if self.direction == LayoutDirection::Horizontal {
                  self.horizontal_alignment
              } else {
                  self.vertical_alignment
              };
      
              let mut main_offset = match main_alignment {
                  LayoutAlignment::Center => main_axis_free_space / 2,
                  LayoutAlignment::End => main_axis_free_space,
                  _ => 0,
              } as i32;
      
              let children_count = self.children.len();
      
              // compute stretched size
              let stretched_size = if main_alignment == LayoutAlignment::Stretch {
                  match self.direction {
                      LayoutDirection::Horizontal => rect.size.width / children_count as u32,
                      LayoutDirection::Vertical => rect.size.height / children_count as u32,
                  }
              } else {
                  0 // just do not stretch
              };
      
              for (i, child) in self.children.iter_mut().enumerate() {
                  let child_bounds = Size::new(rect.size.width, rect.size.height);
                  let mut child_size = child.measure(child_bounds);
      
                  let cross_alignment = if self.direction == LayoutDirection::Horizontal {
                      self.vertical_alignment
                  } else {
                      self.horizontal_alignment
                  };
      
                  match self.direction {
                      LayoutDirection::Horizontal => {
                          if cross_alignment == LayoutAlignment::Stretch {
                              child_size.height = rect.size.height;
                          }
      
                          if main_alignment == LayoutAlignment::Stretch {
                              child_size.width = stretched_size;
                          }
                      }
                      LayoutDirection::Vertical => {
                          if cross_alignment == LayoutAlignment::Stretch {
                              child_size.width = rect.size.width;
                          }
      
                          if main_alignment == LayoutAlignment::Stretch {
                              child_size.height = stretched_size;
                          }
                      }
                  }
      
                  let cross_offset = match self.direction {
                      LayoutDirection::Horizontal => {
                          let free_space = rect.size.height.saturating_sub(child_size.height);
                          match self.vertical_alignment {
                              LayoutAlignment::Center => free_space / 2,
                              LayoutAlignment::End => free_space,
                              _ => 0,
                          }
                      }
                      LayoutDirection::Vertical => {
                          let free_space = rect.size.width.saturating_sub(child_size.width);
                          match self.horizontal_alignment {
                              LayoutAlignment::Center => free_space / 2,
                              LayoutAlignment::End => free_space,
                              _ => 0,
                          }
                      }
                  } as i32;
      
                  let child_rect = match self.direction {
                      LayoutDirection::Horizontal => Rectangle::new(
                          Point::new(
                              rect.top_left.x + main_offset,
                              rect.top_left.y + cross_offset,
                          ),
                          child_size,
                      ),
                      LayoutDirection::Vertical => Rectangle::new(
                          Point::new(
                              rect.top_left.x + cross_offset,
                              rect.top_left.y + main_offset,
                          ),
                          child_size,
                      ),
                  };
      
                  child.layout(child_rect);
      
                  match self.direction {
                      LayoutDirection::Horizontal => {
                          main_offset += child_size.width as i32;
                          if i != children_count - 1 {
                              main_offset += self.gap as i32;
                          }
                      }
                      LayoutDirection::Vertical => {
                          main_offset += child_size.height as i32;
                          if i != children_count - 1 {
                              main_offset += self.gap as i32;
                          }
                      }
                  }
              }
    }

    fn draw(&mut self, framebuffer: &mut FrameBuffer, rect: Rectangle, state: &Self::State) {
        for child in self.children.iter_mut() {
            child.draw(framebuffer);
        }
    }
}