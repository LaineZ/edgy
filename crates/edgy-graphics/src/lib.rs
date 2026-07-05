//#![no_std]

extern crate alloc;

pub mod draw;
pub mod font;
pub mod fonts;
pub mod framebuffer;
pub mod geometry;
pub mod polygon;

use fixed::types::I16F16;
pub type Fixed = I16F16;
