use core::{
    cmp::min,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Integer 2D point in the space
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a point from  X and Y coordinates.
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Creates a point with X and Y equal to zero.
    pub const fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    /// Returns a point with the componentwise absolute value of the coordinates.
    pub const fn abs(self) -> Self {
        Point::new(self.x.abs(), self.y.abs())
    }

    const fn sub_size(self, other: Size) -> Point {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        Point::new(self.x - width, self.y - height)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<Size> for Point {
    type Output = Point;

    /// Offsets a point by adding a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn add(self, other: Size) -> Point {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        Point::new(self.x + width, self.y + height)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign<Size> for Point {
    /// Offsets a point by adding a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn add_assign(&mut self, other: Size) {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        self.x += width;
        self.y += height;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<Size> for Point {
    type Output = Point;

    /// Offsets a point by subtracting a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn sub(self, other: Size) -> Point {
        self.sub_size(other)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl SubAssign<Size> for Point {
    /// Offsets a point by subtracting a size.
    ///
    /// # Panics
    ///
    /// This function will panic if `width` or `height` are too large to be represented as an `i32`
    /// and debug assertions are enabled.
    fn sub_assign(&mut self, other: Size) {
        let width = other.width as i32;
        let height = other.height as i32;

        debug_assert!(width >= 0, "width is too large");
        debug_assert!(height >= 0, "height is too large");

        self.x -= width;
        self.y -= height;
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub const fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }

    pub const fn from_bounding_box(corner_1: Point, corner_2: Point) -> Self {
        let width = (corner_1.x - corner_2.x).unsigned_abs() + 1;
        let height = (corner_1.y - corner_2.y).unsigned_abs() + 1;

        Self { width, height }
    }

    const fn div_u32(self, rhs: u32) -> Size {
        Size::new(self.width / rhs, self.height / rhs)
    }

    pub const fn new_equal(value: u32) -> Self {
        Size {
            width: value,
            height: value,
        }
    }

    pub const fn zero() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Saturating addition.
    ///
    /// Returns `u32::max_value()` for `width` and/or `height` instead of overflowing.
    pub const fn saturating_add(self, other: Self) -> Self {
        Self {
            width: self.width.saturating_add(other.width),
            height: self.height.saturating_add(other.height),
        }
    }

    /// Saturating subtraction.
    ///
    /// Returns `0` for `width` and/or `height` instead of overflowing, if the
    /// value in `other` is larger than in `self`.
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            width: self.width.saturating_sub(other.width),
            height: self.height.saturating_sub(other.height),
        }
    }
}

impl Add for Size {
    type Output = Size;

    fn add(self, other: Size) -> Size {
        Size::new(self.width + other.width, self.height + other.height)
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Size) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl Sub for Size {
    type Output = Size;

    fn sub(self, other: Size) -> Size {
        Size::new(self.width - other.width, self.height - other.height)
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Size) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl Mul<u32> for Size {
    type Output = Size;

    fn mul(self, rhs: u32) -> Size {
        Size::new(self.width * rhs, self.height * rhs)
    }
}

impl MulAssign<u32> for Size {
    fn mul_assign(&mut self, rhs: u32) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl Div<u32> for Size {
    type Output = Size;

    fn div(self, rhs: u32) -> Size {
        self.div_u32(rhs)
    }
}

impl DivAssign<u32> for Size {
    fn div_assign(&mut self, rhs: u32) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

const fn center_offset(size: Size) -> Size {
    size.saturating_sub(Size::new_equal(1)).div_u32(2)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rectangle {
    /// Top left point of the rectangle.
    pub top_left: Point,
    pub size: Size,
}

impl Rectangle {
    pub const fn new(top_left: Point, size: Size) -> Self {
        Rectangle { top_left, size }
    }

    pub const fn zero() -> Rectangle {
        Rectangle::new(Point::zero(), Size::zero())
    }

    pub fn center(&self) -> Point {
        self.top_left + center_offset(self.size)
    }

    pub fn bottom_right(&self) -> Option<Point> {
        if self.size.width > 0 && self.size.height > 0 {
            Some(self.top_left + self.size - Point::new(1, 1))
        } else {
            None
        }
    }

    pub fn union(self, other: Rectangle) -> Rectangle {
        let left = self.top_left.x.min(other.top_left.x);
        let top = self.top_left.y.min(other.top_left.y);

        let right = (self.top_left.x + self.size.width as i32)
            .max(other.top_left.x + other.size.width as i32);

        let bottom = (self.top_left.y + self.size.height as i32)
            .max(other.top_left.y + other.size.height as i32);

        Rectangle::new(
            Point::new(left, top),
            Size::new(
                (right - left) as u32,
                (bottom - top) as u32,
            ),
        )
    }

    pub fn with_corners(corner_1: Point, corner_2: Point) -> Self {
        let left = min(corner_1.x, corner_2.x);
        let top = min(corner_1.y, corner_2.y);

        Rectangle {
            top_left: Point::new(left, top),
            size: Size::from_bounding_box(corner_1, corner_2),
        }
    }

    pub const fn with_center(center: Point, size: Size) -> Self {
        Rectangle {
            top_left: center.sub_size(center_offset(size)),
            size,
        }
    }
}
