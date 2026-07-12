use edgy_graphics::geometry::Size;

#[derive(Clone, Copy, Debug)]
pub struct Constraint {
    pub min_size: Size,
    pub max_size: Size
}

impl Default for Constraint {
    fn default() -> Self {
        Self {
            min_size: Size::zero(),
            max_size: Size::new(u32::MAX, u32::MAX)
        }
    }
}

impl Constraint {
    pub fn tight(size: Size) -> Self {
        Self {
            min_size: size,
            max_size: size,
        }
    }

    pub fn loose(max_size: Size) -> Self {
        Self {
            min_size: Size::zero(),
            max_size
        }
    }

    /// Returns a constrained size
    pub fn constrain(&self, size: Size) -> Size {
        let w = size.width.clamp(self.min_size.width, self.max_size.width);
        let h = size.height.clamp(self.min_size.height, self.max_size.height);
        Size::new(w, h)
    }
}