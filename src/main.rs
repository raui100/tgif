use std::io::Write;

use chrono::Local;
use clap::Parser;
use log::{debug, info, LevelFilter};
use ndarray::Axis;
use ndarray::parallel::prelude::*;
use nshare::ToNdarray2;

mod code;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    src: std::path::PathBuf,

    /// Output image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    dst: std::path::PathBuf,
}

fn main() {
    env_logger::Builder::new()
        .format(move |buf, record| {
            writeln!(
                buf,
                "{}:{} | {} | {} | {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("tgif"), LevelFilter::Debug)
        .init();

    // Use this to artificially reduce the numbers of used CPU threads
    // debug!("Limiting the number of used threads");
    // rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();

    info!("Parsing arguments from CLI");
    let args = Args::parse();

    debug!("Reading the image from disk and converting it into an 2D ndarray");
    let mut image = image::open(args.src)
        .expect("Failed reading input file.")
        .as_luma8()
        .expect("Only use this for 8-bit grayscale pictures")
        .to_owned()
        .into_ndarray2();


    debug!("Calculating the delta of two neighbouring pixels in a row");
    for mut row in image.axis_iter_mut(Axis(0)) {
        let mut prev_pixel: u8 = 0;  // pixel[-1] is defined as 0
        for pixel in row.iter_mut() {
            let delta: u8 = pixel.wrapping_sub(prev_pixel);
            prev_pixel = *pixel;
            *pixel = delta;
        }
    }

    debug!("Creating the Huffman code for each row");
    let code = code::get_code();
    let mut enc: Vec<Vec<bool>> = Vec::new();
    image.axis_iter(Axis(0))
        .into_par_iter()  // Huffman encoding is done in parallel
        .map(|row| {
            let mut vec: Vec<bool> = Vec::new();
            for delta in row.iter() {
                let huffman = &code[*delta as usize];
                vec.extend(huffman);
            }

            // Padding after each row of the image
            // 1. Padding the Huffman coding with 0..7 "1" for byte alignment
            // 2. 32 consecutive "0" to mark the end of the row
            //
            // -> This gives us the ability to encode/decode in parallel properly
            let padding_0 = 32;  // Padding to mark the end of the row
            let padding_1 = 8 - (vec.len() % 8);  // Padding for byte alignment
            vec.extend(vec![true; padding_1]);
            vec.extend(vec![false; padding_0]);

            vec
        })
        .collect_into_vec(&mut enc);

    debug!("Parsing to `BitVec`");
    // let mut bv: BitVec<u8, Msb0> = BitVec::new();
    // for vec in enc.iter() {
    //     bv.extend(vec);
    // }

    let mut enc_u8: Vec<u8> = Vec::new();
    for row in enc.iter() {
        for chunk in row.chunks_exact(8) {
            // Calculates an u8 from [bool; 8] (eg: [1111 1111] -> 255)
            let number: u8 = chunk.iter()
                .fold(0_u8, |value, bool| (value << 1) + (*bool as u8));
            enc_u8.push(number);
        }
    }

    debug!("Writing code to disk");
    let mut file = std::fs::File::create(args.dst).expect("Failed creating destination file");
    file.write_all(&enc_u8).expect("Failed writing to destination file");

    debug!("Finished encoding to TGIF");
}
