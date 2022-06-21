use std::io::Write;

use chrono::Local;
use clap::Parser;
use log::{debug, info, LevelFilter};
use ndarray::{Array2, Axis};
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
        .filter(Some("tgif"), LevelFilter::Trace)
        .init();

    // Use this to artificially reduce the numbers of used CPU threads
    // debug!("Limiting the number of used threads");
    // rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();

    info!("Parsing arguments from CLI");
    let args = Args::parse();

    debug!("Reading the image from disk and converting it into an 2D ndarray");
    let mut image: Array2<u8> = image::open(args.src)
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
    let mut enc: Vec<Vec<u8>> = Vec::new();
    image.axis_iter(Axis(0))
        .into_par_iter()  // Huffman encoding is done in parallel
        .map(|row| {


            // This can be speed up by not copying the underlying bools to vec, but rather using them
            // by ref
            let mut enc_bool: Vec<bool> = Vec::new();
            for delta in row.iter() {
                let huffman = code::HUFFMAN[*delta as usize];
                enc_bool.extend(huffman);
            }

            // 15% faster, but roughly needs to know the image size beforehand
            // use arrayvec::ArrayVec;
            // let mut enc_bool = ArrayVec::<bool, 20_000>::new();
            // for delta in row.iter() {
            //     let huffman = code::HUFFMAN[*delta as usize];
            //     enc_bool.try_extend_from_slice(huffman).unwrap();
            // }

            // Padding after each row of the image
            // 1. Padding the Huffman coding with 0..7 "1" for byte alignment
            // 2. 32 consecutive "0" to mark the end of the row
            //
            // We do this, so we can
            // 1. Encode/decode in parallel easily because 32 consecutive "0" mark the end of a row
            // 2. Cast Vec<bool> to Vec<u8>
            let padding_0 = 32;  // Padding to mark the end of the row
            let padding_1 = 8 - (enc_bool.len() % 8);  // Padding for byte alignment
            enc_bool.extend(vec![true; padding_1]);
            enc_bool.extend(vec![false; padding_0]);

            // Casting the Vec<bool> to Vec<u8>
            let mut enc_u8: Vec<u8> = Vec::new();
            for chunk in enc_bool.chunks_exact(8) {
                // Calculates an u8 from [bool; 8] (eg: [1111 1111] -> 255)
                let number: u8 = chunk.iter()
                    .fold(0_u8, |value, bool| (value << 1) + (*bool as u8));
                enc_u8.push(number);
            }

            enc_u8

        })
        .collect_into_vec(&mut enc);

    debug!("Creating the header");
    // The header has the 4 byte wide entries:
    // 1. The name of the format: TGIF
    // 2. The image width as u32
    // 3. The image height as u32
    // The Huffman encoding schema is fixed
    let header: Vec<u8> = [u32::from_be_bytes(*b"TGIF"), (image.shape()[1] as u32), (image.shape()[0] as u32)].into_iter().flat_map(| v | v.to_be_bytes()).collect();

    debug!("Writing the image tof disk");
    let mut file = std::fs::File::create(args.dst).expect("Failed creating destination file");
    file.write_all(&header).expect("Failed writing image header to disk");
    for row in enc.iter() {
        file.write_all(&row).expect("Failed writing image data to disk");
    }

    debug!("Finished encoding to TGIF");
}
