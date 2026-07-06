#![no_std]

extern crate alloc;

pub mod draw;
pub mod font;
pub mod framebuffer;
pub mod geometry;
pub mod polygon;
pub mod text;

use fixed::types::I16F16;
pub type Fixed = I16F16;
