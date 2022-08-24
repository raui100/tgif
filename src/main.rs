extern crate core;

use std::io::Write;

use clap::Parser;
use log::{debug, info, LevelFilter};
use ndarray::{Array2, Axis};
use nshare::ToNdarray2;

mod args;

fn rice_index(delta: u8) -> u8 {
    if delta <= i8::MAX as u8 {
        delta * 2
    } else {
        (i8::from_be_bytes(delta.to_be_bytes()) as i16 * (-2) - 1).try_into().unwrap()
    }
}

fn main() {
    // Setting up the logging environment
    env_logger::Builder::new()
        .format(move |buf, record| {
            writeln!(
                buf,
                "{}:{} | {} | {} | {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("tgif"), LevelFilter::Trace)
        .init();

    debug!("Parsing arguments from CLI");
    let args = args::Args::parse();

    debug!("Reading the image from disk and converting it into an 2D ndarray");
    let image: Array2<u8> = image::open(&args.src)
        .expect("Failed reading input file.")
        .to_luma8()
        .into_ndarray2();

    // Settings
    let parallel_units: u32 = 4;
    let remainder_bits: u8 = 2;

    // Numbers that are being used a million times
    let chunk_length = image.shape()[0] * parallel_units as usize;

    let mut img_bool: Vec<bool> = Vec::with_capacity(
        image.shape()[0] * image.shape()[1]
    );

    // Iterating over the image
    debug!("Encoding the image as Vec<bool>");
    for rows in image.axis_chunks_iter(Axis(0), parallel_units as usize) {
        let mut prev_pixels = vec![0u8; parallel_units as usize];
        for index in 0..chunk_length {
            let pos_x = index / parallel_units as usize;
            let pos_y = index % parallel_units as usize;
            let prev_pixel = &prev_pixels[pos_y];
            let pixel = rows.get((pos_y, pos_x)).unwrap();
            let delta = prev_pixel.wrapping_sub(*pixel);
            let index = rice_index(delta);
            let unary = index / remainder_bits;
            let remainder = index % remainder_bits;
            prev_pixels[pos_y] = *pixel;

            // Unary coding. Pushes n "1" and one "0" to the vec
            for _ in 0..unary {
                img_bool.push(true);
            }
            img_bool.push(false);

            // Pushing the remainder to the vec
            match remainder {
                0 => img_bool.extend([false, false]),
                1 => img_bool.extend([false, true]),
                2 => img_bool.extend([true, false]),
                3 => img_bool.extend([true, true]),
                _ => panic!("Please extend the matching arm!")
            }
        }
    }

    for _ in 0..(img_bool.len() % 8) {
        img_bool.push(false);  // Padding the end with "0"
    }

    debug!("Encoding the image as Vec<u8>");
    let img_u8: Vec<u8> =
        img_bool.chunks_exact(8)
            .map(|chunk| chunk.iter()
                .fold(0_u8, |value, bool| (value << 1) + (*bool as u8))
            )
            .collect();

    debug!("Writing the image tof disk");
    let mut file = std::fs::File::create(args.dst).expect("Failed creating destination file");

    if !args.no_header {
        debug!("Creating the header");
        // The header has these 4 byte wide entries:
        // 1. The name of the format: TGIF
        // 2. The image width as u32
        // 3. The image height as u32
        // 4. Number of parallel units that have been used during encoding
        let header: Vec<u8> = [
            u32::from_be_bytes(*b"TGIF"),
            image.shape()[1] as u32,
            image.shape()[0] as u32,
            parallel_units as u32,
        ].into_iter()
            .flat_map(|v| v.to_be_bytes())
            .collect();
        file.write_all(&header).expect("Failed writing image header to disk");
    }
    file.write_all(&img_u8).expect("Failed writing the image to disk");

    debug!("Finished encoding to TGIF");
    let compression_rate = img_u8.len() as f64
        / (image.shape()[0] * image.shape()[1]) as f64;

    info!("Achieved {compression_rate:.4} compression rate for {:#?}", &args.src)
}
