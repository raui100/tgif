use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use bit_vec::BitVec;
use clap::Parser;
use huffman_compress::{codebook, CodeBuilder};
use log::{info, LevelFilter};
use ndarray::{Array, Ix2};
use ndarray::prelude::*;
use rayon::prelude::*;

mod decode;
mod code;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    src: PathBuf,

    /// Output image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    dst: PathBuf,
}

fn main() {
    let args = Args::parse();
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} | {} | {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .filter(Some("tgif"), LevelFilter::Debug)
        .init();
    let src = ndarray_image::open_gray_image(args.src).expect("Failed opening source file");
    let src = image::open(args.src)
        .expect("Failed reading input file.")
        .as_luma8()
        .expect("Only use this for 8-bit grayscale pictures")
        .to_owned();
    let (book, tree) = CodeBuilder::from_iter(code::get_weights()).finish();
    let weights = code::get_weights();
    let current = Instant::now();
    let mut prev_pixel: u8 = 0;
    let mut vec_delta: Vec<u32> = Vec::new();
    for (x, y, pixel) in src.enumerate_pixels() {
        if x % src.width() == 0 {
            prev_pixel = 7;
        }
        let delta = pixel[0].wrapping_sub(prev_pixel);
        let huffman_code = weights.get(&delta).unwrap();
        vec_delta.push(*huffman_code);
    }
    let duration = current.elapsed();

    println!("Time elapsed in delta is: {:?}", duration);
    println!("{:?}", &vec_delta.len());
}
