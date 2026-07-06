extern crate alloc;

pub mod draw;
pub mod font;
pub mod framebuffer;
pub mod geometry;
pub mod image;
pub mod polygon;
pub mod text;

use fixed::types::I16F16;
pub type Fixed = I16F16;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PixelFormat {
    Bpp1 = 1,
    Bpp2 = 2,
    Bpp4 = 4,
    Bpp8 = 8,
}

impl PixelFormat {
    pub const fn pixels_per_byte(self) -> u8 {
        8 / self as u8
    }

    pub const fn mask(self) -> u8 {
        ((1u16 << self as u8) - 1) as u8
    }

    pub const fn unpack(self, byte: u8, index: u8) -> u8 {
        let ppb = self.pixels_per_byte();
        let bits = self as u8;
        let shift = (ppb - 1 - index) * bits;

        (byte >> shift) & self.mask()
    }
}

pub const fn parse_palette_rgb888<const N: usize>(s: &str) -> [u32; N] {
    let bytes = s.as_bytes();
    let mut out = [0u32; N];

    let mut i = 0;
    let mut pos = 0;

    while pos < bytes.len() {
        while pos < bytes.len() && (bytes[pos] == b'\n' || bytes[pos] == b'\r') {
            pos += 1;
        }

        if pos >= bytes.len() {
            break;
        }

        assert!(bytes[pos] == b'0');
        assert!(bytes[pos + 1] == b'x');
        pos += 2;

        let mut value = 0;

        while pos < bytes.len() {
            match bytes[pos] {
                b'\n' | b'\r' => break,
                c => {
                    value = (value << 4) | hex(c) as u32;
                    pos += 1;
                }
            }
        }

        out[i] = value;
        i += 1;
    }

    out
}

pub const fn parse_palette_rgb565<const N: usize>(s: &str) -> [u16; N] {
    let rgb888 = parse_palette_rgb888::<N>(s);

    let mut out = [0u16; N];
    let mut i = 0;

    while i < N {
        out[i] = rgb888_to_rgb565(rgb888[i]);
        i += 1;
    }

    out
}

const fn rgb888_to_rgb565(c: u32) -> u16 {
    let r = (c >> 16) & 0xff;
    let g = (c >> 8) & 0xff;
    let b = c & 0xff;

    (((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3)) as u16
}

const fn hex(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("bad hex"),
    }
}
