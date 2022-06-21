use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use bit_vec;
use glob::glob;
use huffman_compress::CodeBuilder;
use ndarray::{Array2, Axis};
use nshare::ToNdarray2;
use rayon::prelude::*;

#[allow(unused, unused_variables)]
/// Counts the number of occurrences of each entry in the iterator
///
/// # Examples
/// ```
/// let input = "aab";
/// let result = HashMap::from([('a', 2), ('b', 1)]);
/// assert_eq!(result, count_frequency(&input.chars()))
/// ```
fn count_frequency<I, K>(iter: &I) -> HashMap<K, usize>
    where
        I: IntoIterator + IntoIterator<Item=K> + Clone,
        K: Eq + Hash,
{
    let mut frequency: HashMap<K, usize> = HashMap::new();
    for item in iter.clone().into_iter() {
        let count = frequency.entry(item).or_insert(0);
        *count += 1;
    }

    frequency
}

#[test]
fn test_count_freq() {
    let input = "aab";
    let result = HashMap::from([('a', 2), ('b', 1)]);
    assert_eq!(result, count_frequency(&input.chars()))
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    RowWise,
    ColumnWise,
}

#[derive(Clone, Debug)]
struct Parameter {
    rle: u8,
    delta: u8,
    raw: u8,
    direction: Direction,
}

#[derive(Clone, Debug)]
struct Score {
    file_name: String,
    dataset: String,
    uncompressed: u64,
    compressed: f64,
    max_png_compressed: f64,
    parameter: Parameter,
}

struct Image {
    path: PathBuf,
    size: u64,
    array: Array2<u8>,
}

fn compress(file: &Image, par: &Parameter) -> Score {
    assert!((0..=8).contains(&par.delta), "Number of bits for delta representation should be within 0..=8");
    assert!([0, 8].contains(&par.raw), "Raw pixel can either be represented or not have 0 or 8 bits");
    assert!((par.delta == 8) || (par.raw == 8), "Either delta or raw must have 8 bit so that every pixel can be represented");

    // 1..4097 can be used by RLE
    // 4097...8192 can be used by delta - it will never use more than 4096..4351 though (255 values for u8)
    // 8182..16_384 can be used by raw pixel - it will never use more than 8_192..8_447 though (255 values for u8)
    let (prefix_rle, prefix_delta, prefix_raw) = match (par.rle > 0, par.delta > 0, par.raw > 0) {
        (false, false, true) => (None, None, Some(2_u32.pow(13))),
        (false, true, false) => (None, Some(2_u32.pow(12) + 1), None),
        (false, true, true) => (None, Some(2_u32.pow(12) + 1), Some(2_u32.pow(13))),
        (true, false, true) => (Some(0_u32), None, Some(2_u32.pow(13))),
        (true, true, false) => (Some(0), Some(2_u32.pow(12) + 1), None),
        (true, true, true) => (Some(0), Some(2_u32.pow(12) + 1), Some(2_u32.pow(13))),
        _ => unreachable!("Can't be reached due to assert above"),
    };
    let (prefix_rle, prefix_delta, prefix_raw) = (prefix_rle.unwrap(), prefix_delta.unwrap(), prefix_raw.unwrap());
    // With 1 bit we can represent [1, 2].
    let rle_max: u32 = 2_u32.pow(par.rle as u32);

    // The set of numbers we can represent with the number depends on the number of bits we use for RLE
    let delta_vec: Vec<u8> = match par.rle > 0 {
        // If we use 1 or more bits for RLE, delta doesn't have to represent "0"
        // With 1 bit we can represent [-1, 1] -> [255, 1] in u8
        true => (-2_i32.pow(par.delta as u32)..2_i32.pow(par.delta as u32)).map(|n| ((n + 256) % 256) as u8).filter(|n| n != &0).collect(),
        // If there is no RLE we have to represent the "0" if necessary
        // With 1 bit we can represent [-1, 0] -> [255, 0] in u8
        false => (-2_i32.pow(par.delta as u32)..2_i32.pow(par.delta as u32) - 1).map(|n| ((n + 256) % 256) as u8).collect(),
    };

    let axis = match par.direction {
        Direction::RowWise => 0,
        Direction::ColumnWise => 1,
    };

    let mut encoded: Vec<u32> = Vec::new();
    for array in file.array.axis_iter(Axis(axis)) {
        let mut prev_pixel: u8 = 0;  // The 0th pixel is defined as 0 (black)
        let mut rle_counter: u32 = 0;

        for pixel in array {
            // Run Length Encoding - Has the highest priority
            if par.rle > 0 {

                // Checks if RLE is applicable
                // 1. The previous pixel and the current pixel are equal
                // 2. The number of RLE that has already occurred can be represented with the available bits
                if pixel == &prev_pixel {
                    // "Storing" the RLE
                    rle_counter += 1;

                    // Checking if the max of the counter has been reached
                    if rle_counter == rle_max {
                        // Encoding the RLE count
                        let number = prefix_rle + rle_counter;
                        encoded.push(number);

                        // Resetting the state
                        rle_counter = 0;
                    }

                    continue;  // Continuing to the next pixel
                } else if rle_counter > 0 {
                    // Encoding the RLE count
                    let number = prefix_rle + rle_counter;
                    encoded.push(number);

                    // Resetting the state
                    rle_counter = 0;
                }
            }

            // Delta encoding
            if par.delta > 0 {
                let delta = prev_pixel.wrapping_sub(*pixel);
                if delta_vec.contains(&delta) {
                    // Storing the delta of the current pixel
                    let number = prefix_delta + delta as u32;
                    encoded.push(number);

                    // Taking care of the state
                    prev_pixel = *pixel;
                    // Continuing to the next pixel
                    continue;
                }
            }

            // Raw encoding of the pixel. Least desirable
            if par.raw > 0 {
                let number = prefix_raw + *pixel as u32;
                encoded.push(number);

                // Taking care of the state
                prev_pixel = *pixel;
                // Continuing to the next pixel
                continue;
            }

            unreachable!("A pixel has been gone under!\nParameter: {:#?}\nPixel: {}\nPrevious Pixel: {}", par, pixel, prev_pixel);
        }
    }

    let freq = count_frequency(&encoded);
    let (book, _) = CodeBuilder::from_iter(freq).finish();
    let mut compressed = bit_vec::BitVec::new();
    for number in encoded {
        book.encode(&mut compressed, &number).unwrap();
    }
    let dataset = {
        if file.path.to_string_lossy().contains("/dark/") {
            "dark"
        } else if file.path.to_string_lossy().contains("/psa/") {
            "psa"
        } else if file.path.to_string_lossy().contains("/shoesole/") {
            "shoesole"
        } else {
            panic!("Unknown dataset")
        }
    };


    Score {
        file_name: file.path.file_name().unwrap().to_string_lossy().to_string(),
        dataset: dataset.to_string(),
        parameter: par.clone(),
        uncompressed: (file.array.len() * 8) as u64,
        compressed: 100.0 * (compressed.len() as f64 / (file.array.len() * 8) as f64),
        max_png_compressed: 100.0 * (file.size * 8) as f64 / (file.array.len() * 8) as f64,
    }
}

fn main() {
    let current = Instant::now();

    // Finding paths to all PNG images at the specified directory
    let images: Vec<PathBuf> = glob("/home/private/Documents/Uni/Bachelorarbeit/assets/pics/**/*.png")
        .expect("Failed to read glob pattern")
        .map(|result| result.unwrap())
        .collect();

    let rle = 0..=12;  // RLE encoding can have 0 to 12 bits (=4096). This is enough to encode a whole row/column
    let delta = 0..=8;  // Delta can have 0 to 8 bits
    let raw = [0, 8];  // Raw pixel are either represented fully or not (=0 bits)
    let directions: [Direction; 2] = [Direction::RowWise, Direction::ColumnWise];  // The image can be compressed row by row or column by column

    let parameters: Vec<Parameter> = itertools::iproduct!(rle, delta, raw, directions)
        .map(|(rle, delta, raw, direction)| Parameter { rle, delta, raw, direction })
        .filter(|par| (par.delta == 8) || (par.raw == 8))  // Either delta or raw must be 8 bit, so every pixel can be represented
        .collect();

    // Preparing the results file
    let _ = std::fs::remove_file("results.csv");
    File::create("results.csv").expect("Unable to create file");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true) // This is needed to append to file
        .open("results.csv")
        .unwrap();
    file.write_all(b"INDEX,NAME,DATASET,UNCOMPRESSED,MAX_PNG_COMPRESSED,COMPRESSION,RLE,DELTA,RAW\n").unwrap();

    let mut index: u32 = 0;
    for path in images.iter() {
        println!("Processing image: {:?}", &path);
        let image = Image {
            path: path.clone(),
            size: std::fs::metadata(path.clone()).unwrap().len(),
            array: image::open(path)
                .expect("Failed reading input file.")
                .as_luma8()
                .expect("Only use this for 8-bit grayscale pictures")
                .to_owned()
                .into_ndarray2(),
        };
        let scores: Vec<Score> = parameters.par_iter().map(|par| compress(&image, par)).collect();
        for score in scores {
            let out = format!("{},{},{},{},{:.3},{:.3},{},{},{},{:?}\n",
                              index,
                              score.file_name,
                              score.dataset,
                              score.uncompressed,
                              score.max_png_compressed,
                              score.compressed,
                              score.parameter.rle,
                              score.parameter.delta,
                              score.parameter.raw,
                              score.parameter.direction,
            );
            file.write_all(out.as_ref()).unwrap();
            index += 1;
        }
    }

    let duration = current.elapsed();
    println!("Finished parameter study after: {:.2?}", duration);
}