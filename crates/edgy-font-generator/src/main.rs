use std::io::Write as IoWrite;
use std::path::PathBuf;
use std::{fmt::Write, fs::File};

use clap::Parser;

use crate::providers::FontRasterizerProvider;
use crate::providers::bdf::BdfProvider;
use crate::providers::fontdue::FontdueProvider;

pub mod providers;

/// Font converter & generator for the edgy library
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input font file. Supports *.ttf, *.otf, *.bdf fonts
    input: PathBuf,
    /// Output Rust source file
    output: PathBuf,
    /// Rasterized vector font size in pixels. Ignored when input is *.bdf font
    #[arg(short, long, default_value_t = 10)]
    size: u8,
    #[arg(short, long, default_value_t = 128)]
    /// Antialiasing threashold for rasterized vector fonts. Ignored when input is *.bdf font
    alpha_threashold: u8,
    /// Characters to be included in the font. If unspecified, font will include entire ASCII character set
    #[arg(short, long)]
    characters: Option<String>,
}

fn main() {
    let args = Args::parse();
    let stem = args.output.file_stem().unwrap().to_string_lossy();

    if !stem.chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
        panic!(
            "Invalid output file name. File name can contain only ASCII letters and `_` characters"
        );
    }

    println!("Rasterizing font: {}", args.input.display());

    let provider: Box<dyn FontRasterizerProvider> =
        match args.input.extension().and_then(|e| e.to_str()) {
            Some("ttf") | Some("otf") => Box::new(FontdueProvider::new(
                args.input.clone(),
                args.size,
                args.alpha_threashold,
            )),

            Some("bdf") => Box::new(BdfProvider::new(args.input.clone())),

            Some(ext) => panic!("Unsupported font format: {ext}"),

            _ => panic!("Cannot infer font format..."),
        };

    let mut bitmap_data = Vec::new();
    let mut out = String::new();
    out.push_str("use edgy_graphics::font::{Font, Glyph};\n\n");

    let chars = args
        .characters
        .unwrap_or_else(|| (32u8..=126).map(char::from).collect());
    let chars_vec: Vec<char> = chars.chars().collect();

    writeln!(out, "const GLYPHS: [Glyph; {}] = [", chars_vec.len()).unwrap();

    for char in chars_vec {
        let offset = bitmap_data.len();
        let bitmap = provider.rasterize(char);
        let metrics = provider.get_glyph_data(char, offset);
        let stride = metrics.width.div_ceil(8) as usize;
        
        bitmap_data.resize(offset + stride * metrics.height as usize, 0);
        
        for y in 0..metrics.height as usize {
            for x in 0..metrics.width as usize {
                if bitmap[y * metrics.width as usize + x] == 1 {
                    let byte = offset + y * stride + x / 8;
                    bitmap_data[byte] |= 1 << (7 - (x % 8));
                }
            }
        }

        writeln!(
            out,
            "    Glyph {{ character: {:?}, offset: {}, width: {}, height: {}, x_offset: {}, y_offset: {}, advance_width: {} }},",
            char,
            offset,
            metrics.width,
            metrics.height,
            metrics.x_offset as i8,
            metrics.y_offset as i8,
            metrics.advance_width as u8
        ).unwrap();

        println!(
            "{}: offset={}, {}x{} advance width: {}",
            char, offset, metrics.width, metrics.height, metrics.advance_width
        );
    }
    out.push_str("];\n\n");

    writeln!(out, "const FONT_BITMAP: [u8; {}] = [", bitmap_data.len()).unwrap();

    for chunk in bitmap_data.chunks(32) {
        out.push_str("    ");

        for b in chunk {
            write!(out, "0x{:02X}, ", b).unwrap();
        }

        out.push('\n');
    }

    out.push_str("];\n\n");
    writeln!(
        out,
        "pub const {}: Font = Font {{
        glyphs: &GLYPHS,
        bitmap: &FONT_BITMAP,\n}};\n",
        stem.to_uppercase()
    )
    .unwrap();

    let mut file = File::create(args.output.clone()).unwrap();
    file.write_all(&out.as_bytes()).unwrap();
    println!("Font rasterized at: {}", args.output.display());
}
