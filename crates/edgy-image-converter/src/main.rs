use std::io::Write as IoWrite;
use std::{fs::File, path::PathBuf};

use owo_colors::OwoColorize;
use std::fmt::Write;
use clap::{Parser, ValueEnum};
use edgy_graphics::framebuffer::FrameBuffer;
use embedded_heatshrink::{HSEFinishRes, HSEPollRes, HSESinkRes, HeatshrinkEncoder};
use image::{ImageBuffer, ImageReader, Rgb,
    imageops::{self, BiLevel, ColorMap,},
};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ImageConvertType {
    BinaryColor,
    Palette,
}

impl ToString for ImageConvertType {
    fn to_string(&self) -> String {
        match self {
            ImageConvertType::BinaryColor => String::from("binary-color"),
            ImageConvertType::Palette => String::from("palette"),
        }
    }
}

/// Image converter for the edgy library
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input image file file.
    input: PathBuf,
    /// Output Rust source file
    output: PathBuf,
    /// Palette .hex file path. If unspecified, converation processed in binary color (monochrome) mode
    #[arg(long)]
    palette: Option<PathBuf>,
    /// Applies heatshrink compression algorithm for the image data
    #[arg(short, long, default_value_t = false)]
    compress: bool,
    /// Enable image dithering
    #[arg(short, long, default_value_t = false)]
    dither: bool,
    /// Exclude palette indexes from using when converting image.
    #[arg(long, value_delimiter = ' ')]
    exclude_indexes: Vec<u8>,
}

#[derive(Default)]
struct Palette {
    colors: Vec<Rgb<u8>>,
}

impl ColorMap for Palette {
    type Color = Rgb<u8>;

    fn index_of(&self, color: &Rgb<u8>) -> usize {
        let mut best = 0;
        let mut best_dist = u32::MAX;

        for (i, p) in self.colors.iter().enumerate() {
            let dr = color[0] as i32 - p[0] as i32;
            let dg = color[1] as i32 - p[1] as i32;
            let db = color[2] as i32 - p[2] as i32;

            let dist = (dr * dr + dg * dg + db * db) as u32;

            if dist < best_dist {
                best_dist = dist;
                best = i;
            }
        }

        best
    }

    fn map_color(&self, color: &mut Rgb<u8>) {
        let i = self.index_of(color);
        *color = self.colors[i];
    }
}

fn compress(data: &[u8]) -> Option<Vec<u8>> {
    let mut encoder = HeatshrinkEncoder::new(8, 4).unwrap();
    let mut out = Vec::new();
    let mut tmp = [0u8; 256];

    let mut input = data;

    while !input.is_empty() {
        match encoder.sink(input) {
            HSESinkRes::Ok(n) => input = &input[n..],
            _ => return None,
        }
        
        loop {
            match encoder.poll(&mut tmp) {
                HSEPollRes::More(n) => {
                    out.extend_from_slice(&tmp[..n]);
                }

                HSEPollRes::Empty(n) => {
                    out.extend_from_slice(&tmp[..n]);
                    break;
                }

                _ => return None,
            }
        }
    }

    loop {
        match encoder.finish() {
            HSEFinishRes::Done => break,

            HSEFinishRes::More => loop {
                match encoder.poll(&mut tmp) {
                    HSEPollRes::More(n) => {
                        out.extend_from_slice(&tmp[..n]);
                    }

                    HSEPollRes::Empty(n) => {
                        out.extend_from_slice(&tmp[..n]);
                        break;
                    }

                    _ => return None,
                }
            },
            _ => return None,
        }
    }

    if out.len() < data.len() {
        Some(out)
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();
    if let Err(error) = main_inner(args) {
        eprintln!("error: {:?}", error);
        std::process::exit(1);
    }
}

fn generate_const_data(out: &mut String, data: &[u8]) {
    writeln!(out, "const BITMAP: [u8; {}] = [", data.len()).unwrap();
    for chunk in data.chunks(32) {
        out.push_str("    ");
        for b in chunk {
            write!(out, "0x{:02X}, ", b).unwrap();
        }
        out.push('\n');
    }
    out.push_str("];\n\n");
}

fn main_inner(args: Args) -> anyhow::Result<()> {
    let image = ImageReader::open(&args.input)
        .map_err(|op| anyhow::anyhow!("Failed to open input image: {}", op))?
        .decode()
        .map_err(|op| anyhow::anyhow!("Failed to decode input image: {}", op))?;

    let stem = args.output.file_stem().unwrap().to_string_lossy();
    
    if !stem.chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
        return Err(anyhow::anyhow!(
            "Invalid output file name. File name can contain only ASCII letters and `_` characters"
        ))
    }
    
    let mut out = String::new();
    let mut enable_compression = args.compress;
    out.push_str("use edgy_graphics::image::Image;\n");
    out.push_str("use edgy_graphics::PixelFormat;\n\n");
    
    if let Some(palette_path) = args.palette {
        unimplemented!()
    } else {
        println!("{} {}", "Image:".bold(), args.input.display());
        
        // 1 bit
        let grayscale = image.to_luma8();
        let mut grayscale_buffer = ImageBuffer::from(grayscale);
        if args.dither {
            imageops::dither(&mut grayscale_buffer, &BiLevel);   
        }
        let mut framebuffer = FrameBuffer::new(
            image.width() as u16,
            image.height() as u16,
            edgy_graphics::PixelFormat::Bpp1,
        );
        
        for (x, y, pixel) in grayscale_buffer.enumerate_pixels() {
            if pixel[0] >= 128 {
                framebuffer.set_pixel(x as u16, y as u16, 1);
            }
        }

        let data_size = &framebuffer.data.len();
        if !args.compress {
            generate_const_data(&mut out, &framebuffer.data);   
        } else {
            println!("Compressing: {data_size} bytes...");
            if let Some(compressed_data) = compress(&framebuffer.data) {
                generate_const_data(&mut out, &compressed_data);
                println!("Compressed size {} bytes", &compressed_data.len());
            } else {
                eprintln!("{}", "Compression is retarded - writting with disabled compression...".bold().yellow());
                enable_compression = false;
                generate_const_data(&mut out, &framebuffer.data);   
            }
        }
    }

    writeln!(out, "pub const {}: Image = Image {{ 
        bitmap: &BITMAP,
        width: {},
        height: {},
        format: PixelFormat::Bpp1,
        compress: {}
}};", 
    stem.to_uppercase(), image.width(), image.height(), enable_compression.to_string())?;

    let mut file = File::create(args.output.clone()).unwrap();
    file.write_all(&out.as_bytes()).unwrap();
    println!("{} Done! output file written to: {}", '✓'.green(), args.output.display().bold());
    Ok(())
}
