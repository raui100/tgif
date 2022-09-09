use log::{debug, info, trace};

use crate::args::Args;

pub fn decode(args: &Args) {
    debug!("Reading the TGIF file from disk");
    let tgif = std::fs::read(&args.src)
        .unwrap_or_else(|_| panic!("Failed reading {}", &args.src));

    debug!("Parsing the header");
    let header = parse_header(&tgif, args);

    debug!("Decoding Rice-code to Rice-index");
    let unordered_rice_indices = decode_rice(
        &tgif[header.start_index..],
        header.remainder,
        (header.width * header.height) as usize,
    );

    debug!("Reordering the Rice-index");
    let ordered_rice_indices = reorder_img(
        unordered_rice_indices,
        header.pae as usize,
        header.width as usize,
    );

    debug!("Transforming the Rice-Index to deltas");
    let deltas = reverse_rice_index(ordered_rice_indices);

    debug!("Transforming the delta back to the original image");
    let img_u8 = reverse_delta(
        deltas,
        header.width as usize,
    );

    debug!("Saving the original image to disk");
    image::save_buffer(
        &args.dst,
        &img_u8,
        header.width,
        header.height,
        image::ColorType::L8,
    )
        .unwrap();

    info!("Finished!")
}

/// Reverse index
fn reverse_rice_index(mut vec: Vec<u8>) -> Vec<u8> {
    for num in vec.iter_mut() {
        *num = rev_rice_index(*num)
    }

    vec
}

#[inline(always)]
fn rev_rice_index(index: u8) -> u8 { REV_RICE_INDEX[index as usize] }

const REV_RICE_INDEX: [u8; 256] = [0, 255, 1, 254, 2, 253, 3, 252, 4, 251, 5, 250, 6, 249, 7, 248, 8, 247, 9, 246, 10, 245, 11, 244, 12, 243, 13, 242, 14, 241, 15, 240, 16, 239, 17, 238, 18, 237, 19, 236, 20, 235, 21, 234, 22, 233, 23, 232, 24, 231, 25, 230, 26, 229, 27, 228, 28, 227, 29, 226, 30, 225, 31, 224, 32, 223, 33, 222, 34, 221, 35, 220, 36, 219, 37, 218, 38, 217, 39, 216, 40, 215, 41, 214, 42, 213, 43, 212, 44, 211, 45, 210, 46, 209, 47, 208, 48, 207, 49, 206, 50, 205, 51, 204, 52, 203, 53, 202, 54, 201, 55, 200, 56, 199, 57, 198, 58, 197, 59, 196, 60, 195, 61, 194, 62, 193, 63, 192, 64, 191, 65, 190, 66, 189, 67, 188, 68, 187, 69, 186, 70, 185, 71, 184, 72, 183, 73, 182, 74, 181, 75, 180, 76, 179, 77, 178, 78, 177, 79, 176, 80, 175, 81, 174, 82, 173, 83, 172, 84, 171, 85, 170, 86, 169, 87, 168, 88, 167, 89, 166, 90, 165, 91, 164, 92, 163, 93, 162, 94, 161, 95, 160, 96, 159, 97, 158, 98, 157, 99, 156, 100, 155, 101, 154, 102, 153, 103, 152, 104, 151, 105, 150, 106, 149, 107, 148, 108, 147, 109, 146, 110, 145, 111, 144, 112, 143, 113, 142, 114, 141, 115, 140, 116, 139, 117, 138, 118, 137, 119, 136, 120, 135, 121, 134, 122, 133, 123, 132, 124, 131, 125, 130, 126, 129, 127, 128];

#[test]
fn test_reverse_rice_index() {
    let original = vec![0, 1, 255, 127, 128];
    let rice_index = vec![0, 2, 1, 254, 255];
    assert_eq!(reverse_rice_index(rice_index), original);
}

/// Reverses the delta calculation
fn reverse_delta(mut delta: Vec<u8>, img_width: usize) -> Vec<u8> {
    for chunk in delta.chunks_exact_mut(img_width) {
        let mut prev_num = 0_u8;
        for delta in chunk.iter_mut() {
            prev_num = prev_num.wrapping_sub(*delta);
            *delta = prev_num
        }
    }

    delta
}

#[test]
fn test_reverse_delta() {
    // Most simple case
    assert_eq!(reverse_delta(vec![0], 1), vec![0]);

    // Simple case with wrapping sub
    assert_eq!(reverse_delta(vec![1], 1), vec![255]);

    // Two wrapping subs
    assert_eq!(reverse_delta(vec![255, 255], 2), vec![1, 2]);

    // Not wrapping, wrapping, not wrapping, wrapping
    assert_eq!(reverse_delta(vec![0, 1, 254, 3], 4), vec![0, 255, 1, 254]);

    // Delta with reset
    assert_eq!(
        reverse_delta(vec![1, 0, 3, 254], 2),
        vec![255, 255, 253, 255]
    );

    // Another delta with reset
    let original = vec![0, 0, 255, 255, 0, 0, 0, 255];
    let delta = vec![0, 0, 1, 0, 0, 0, 0, 1];
    assert_eq!(reverse_delta(delta, 2), original)
}


/// Reorders a vec that had been built with `parallel_encoding_units`
///
/// # Example
/// Let's say we an image with 2 pixel width and 3 pixel height: [[0, 1], [2, 3], [4, 5]]
/// This image is being reordered with 3 parallel encoding units to: [0, 2, 4, 1, 3, 5]
/// To reorder the image we calculate the original indices as following:
/// We can process independent chunks of the image that have the `chunk_size` image width times
/// number of parallel encoding units, because we now that the first `parallel_encoding_units` rows
/// are completely independent of all following rows
/// In our example `chunk_size = 2 * 3 = 6`
/// Within those chunks the indices that map the reordered image to the original image can be split
/// mentally in `positions` and `offsets`.
/// The `positions` point to the indices of the first row of pixels in the ordered image. In our
/// example this would be `positions = [0, 3, 0, 3, 0, 3]`.
/// The `offsets` define the row within the chunk. `positions` always points to the first row of
/// pixels. With the offset being 1 we would point to the second row. In our example this would be
/// `offsets = [0, 0, 1, 1, 2, 2]`
/// When we sum the elements of `positions
fn reorder_img(img: Vec<u8>, parallel_encoding_units: usize, img_width: usize) -> Vec<u8> {
    // Testing if reordering is unnecessary
    if parallel_encoding_units == 1 || img_width == 1 { return img; }

    let chunk_size = parallel_encoding_units * img_width;

    let indices = (0..chunk_size)
        .map(|ind| {
            let position = (ind * parallel_encoding_units) % chunk_size;
            let offset = ind / img_width;
            position + offset
        })
        .collect::<Vec<usize>>();

    reorder_by_mapping(&img, &indices, chunk_size)
}

fn reorder_by_mapping(img: &[u8], indices: &[usize], chunk_size: usize) -> Vec<u8> {
    img.chunks_exact(chunk_size)
        .flat_map(|chunk| indices.iter().map(|ind| chunk[*ind]))
        .collect()
}

#[test]
fn test_reorder_img() {
    // One Row
    let ordered = vec![0, 1, 2, 3];
    assert_eq!(reorder_img(ordered.clone(), 1, 4), ordered);

    // Two rows but `img_width=1`
    let ordered = vec![0, 1];
    assert_eq!(reorder_img(ordered.clone(), 2, 1), ordered);

    // 4 rows but `img_width=1`
    let ordered = vec![0, 1, 2, 3];
    assert_eq!(reorder_img(ordered.clone(), 2, 1), ordered);

    // Actually ordering stuff
    let ordered = vec![0, 1, 2, 3];
    let unordered = vec![0, 2, 1, 3];
    assert_eq!(reorder_img(unordered, 2, 2), ordered)
}


#[inline(always)]
fn u8_to_array_bool(num: u8) -> &'static [bool; 8] {
    &U8_TO_ARRAY_BOOL[num as usize]
}

const U8_TO_ARRAY_BOOL: [[bool; 8]; 256] = [[false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, true], [false, false, false, false, false, false, true, false], [false, false, false, false, false, false, true, true], [false, false, false, false, false, true, false, false], [false, false, false, false, false, true, false, true], [false, false, false, false, false, true, true, false], [false, false, false, false, false, true, true, true], [false, false, false, false, true, false, false, false], [false, false, false, false, true, false, false, true], [false, false, false, false, true, false, true, false], [false, false, false, false, true, false, true, true], [false, false, false, false, true, true, false, false], [false, false, false, false, true, true, false, true], [false, false, false, false, true, true, true, false], [false, false, false, false, true, true, true, true], [false, false, false, true, false, false, false, false], [false, false, false, true, false, false, false, true], [false, false, false, true, false, false, true, false], [false, false, false, true, false, false, true, true], [false, false, false, true, false, true, false, false], [false, false, false, true, false, true, false, true], [false, false, false, true, false, true, true, false], [false, false, false, true, false, true, true, true], [false, false, false, true, true, false, false, false], [false, false, false, true, true, false, false, true], [false, false, false, true, true, false, true, false], [false, false, false, true, true, false, true, true], [false, false, false, true, true, true, false, false], [false, false, false, true, true, true, false, true], [false, false, false, true, true, true, true, false], [false, false, false, true, true, true, true, true], [false, false, true, false, false, false, false, false], [false, false, true, false, false, false, false, true], [false, false, true, false, false, false, true, false], [false, false, true, false, false, false, true, true], [false, false, true, false, false, true, false, false], [false, false, true, false, false, true, false, true], [false, false, true, false, false, true, true, false], [false, false, true, false, false, true, true, true], [false, false, true, false, true, false, false, false], [false, false, true, false, true, false, false, true], [false, false, true, false, true, false, true, false], [false, false, true, false, true, false, true, true], [false, false, true, false, true, true, false, false], [false, false, true, false, true, true, false, true], [false, false, true, false, true, true, true, false], [false, false, true, false, true, true, true, true], [false, false, true, true, false, false, false, false], [false, false, true, true, false, false, false, true], [false, false, true, true, false, false, true, false], [false, false, true, true, false, false, true, true], [false, false, true, true, false, true, false, false], [false, false, true, true, false, true, false, true], [false, false, true, true, false, true, true, false], [false, false, true, true, false, true, true, true], [false, false, true, true, true, false, false, false], [false, false, true, true, true, false, false, true], [false, false, true, true, true, false, true, false], [false, false, true, true, true, false, true, true], [false, false, true, true, true, true, false, false], [false, false, true, true, true, true, false, true], [false, false, true, true, true, true, true, false], [false, false, true, true, true, true, true, true], [false, true, false, false, false, false, false, false], [false, true, false, false, false, false, false, true], [false, true, false, false, false, false, true, false], [false, true, false, false, false, false, true, true], [false, true, false, false, false, true, false, false], [false, true, false, false, false, true, false, true], [false, true, false, false, false, true, true, false], [false, true, false, false, false, true, true, true], [false, true, false, false, true, false, false, false], [false, true, false, false, true, false, false, true], [false, true, false, false, true, false, true, false], [false, true, false, false, true, false, true, true], [false, true, false, false, true, true, false, false], [false, true, false, false, true, true, false, true], [false, true, false, false, true, true, true, false], [false, true, false, false, true, true, true, true], [false, true, false, true, false, false, false, false], [false, true, false, true, false, false, false, true], [false, true, false, true, false, false, true, false], [false, true, false, true, false, false, true, true], [false, true, false, true, false, true, false, false], [false, true, false, true, false, true, false, true], [false, true, false, true, false, true, true, false], [false, true, false, true, false, true, true, true], [false, true, false, true, true, false, false, false], [false, true, false, true, true, false, false, true], [false, true, false, true, true, false, true, false], [false, true, false, true, true, false, true, true], [false, true, false, true, true, true, false, false], [false, true, false, true, true, true, false, true], [false, true, false, true, true, true, true, false], [false, true, false, true, true, true, true, true], [false, true, true, false, false, false, false, false], [false, true, true, false, false, false, false, true], [false, true, true, false, false, false, true, false], [false, true, true, false, false, false, true, true], [false, true, true, false, false, true, false, false], [false, true, true, false, false, true, false, true], [false, true, true, false, false, true, true, false], [false, true, true, false, false, true, true, true], [false, true, true, false, true, false, false, false], [false, true, true, false, true, false, false, true], [false, true, true, false, true, false, true, false], [false, true, true, false, true, false, true, true], [false, true, true, false, true, true, false, false], [false, true, true, false, true, true, false, true], [false, true, true, false, true, true, true, false], [false, true, true, false, true, true, true, true], [false, true, true, true, false, false, false, false], [false, true, true, true, false, false, false, true], [false, true, true, true, false, false, true, false], [false, true, true, true, false, false, true, true], [false, true, true, true, false, true, false, false], [false, true, true, true, false, true, false, true], [false, true, true, true, false, true, true, false], [false, true, true, true, false, true, true, true], [false, true, true, true, true, false, false, false], [false, true, true, true, true, false, false, true], [false, true, true, true, true, false, true, false], [false, true, true, true, true, false, true, true], [false, true, true, true, true, true, false, false], [false, true, true, true, true, true, false, true], [false, true, true, true, true, true, true, false], [false, true, true, true, true, true, true, true], [true, false, false, false, false, false, false, false], [true, false, false, false, false, false, false, true], [true, false, false, false, false, false, true, false], [true, false, false, false, false, false, true, true], [true, false, false, false, false, true, false, false], [true, false, false, false, false, true, false, true], [true, false, false, false, false, true, true, false], [true, false, false, false, false, true, true, true], [true, false, false, false, true, false, false, false], [true, false, false, false, true, false, false, true], [true, false, false, false, true, false, true, false], [true, false, false, false, true, false, true, true], [true, false, false, false, true, true, false, false], [true, false, false, false, true, true, false, true], [true, false, false, false, true, true, true, false], [true, false, false, false, true, true, true, true], [true, false, false, true, false, false, false, false], [true, false, false, true, false, false, false, true], [true, false, false, true, false, false, true, false], [true, false, false, true, false, false, true, true], [true, false, false, true, false, true, false, false], [true, false, false, true, false, true, false, true], [true, false, false, true, false, true, true, false], [true, false, false, true, false, true, true, true], [true, false, false, true, true, false, false, false], [true, false, false, true, true, false, false, true], [true, false, false, true, true, false, true, false], [true, false, false, true, true, false, true, true], [true, false, false, true, true, true, false, false], [true, false, false, true, true, true, false, true], [true, false, false, true, true, true, true, false], [true, false, false, true, true, true, true, true], [true, false, true, false, false, false, false, false], [true, false, true, false, false, false, false, true], [true, false, true, false, false, false, true, false], [true, false, true, false, false, false, true, true], [true, false, true, false, false, true, false, false], [true, false, true, false, false, true, false, true], [true, false, true, false, false, true, true, false], [true, false, true, false, false, true, true, true], [true, false, true, false, true, false, false, false], [true, false, true, false, true, false, false, true], [true, false, true, false, true, false, true, false], [true, false, true, false, true, false, true, true], [true, false, true, false, true, true, false, false], [true, false, true, false, true, true, false, true], [true, false, true, false, true, true, true, false], [true, false, true, false, true, true, true, true], [true, false, true, true, false, false, false, false], [true, false, true, true, false, false, false, true], [true, false, true, true, false, false, true, false], [true, false, true, true, false, false, true, true], [true, false, true, true, false, true, false, false], [true, false, true, true, false, true, false, true], [true, false, true, true, false, true, true, false], [true, false, true, true, false, true, true, true], [true, false, true, true, true, false, false, false], [true, false, true, true, true, false, false, true], [true, false, true, true, true, false, true, false], [true, false, true, true, true, false, true, true], [true, false, true, true, true, true, false, false], [true, false, true, true, true, true, false, true], [true, false, true, true, true, true, true, false], [true, false, true, true, true, true, true, true], [true, true, false, false, false, false, false, false], [true, true, false, false, false, false, false, true], [true, true, false, false, false, false, true, false], [true, true, false, false, false, false, true, true], [true, true, false, false, false, true, false, false], [true, true, false, false, false, true, false, true], [true, true, false, false, false, true, true, false], [true, true, false, false, false, true, true, true], [true, true, false, false, true, false, false, false], [true, true, false, false, true, false, false, true], [true, true, false, false, true, false, true, false], [true, true, false, false, true, false, true, true], [true, true, false, false, true, true, false, false], [true, true, false, false, true, true, false, true], [true, true, false, false, true, true, true, false], [true, true, false, false, true, true, true, true], [true, true, false, true, false, false, false, false], [true, true, false, true, false, false, false, true], [true, true, false, true, false, false, true, false], [true, true, false, true, false, false, true, true], [true, true, false, true, false, true, false, false], [true, true, false, true, false, true, false, true], [true, true, false, true, false, true, true, false], [true, true, false, true, false, true, true, true], [true, true, false, true, true, false, false, false], [true, true, false, true, true, false, false, true], [true, true, false, true, true, false, true, false], [true, true, false, true, true, false, true, true], [true, true, false, true, true, true, false, false], [true, true, false, true, true, true, false, true], [true, true, false, true, true, true, true, false], [true, true, false, true, true, true, true, true], [true, true, true, false, false, false, false, false], [true, true, true, false, false, false, false, true], [true, true, true, false, false, false, true, false], [true, true, true, false, false, false, true, true], [true, true, true, false, false, true, false, false], [true, true, true, false, false, true, false, true], [true, true, true, false, false, true, true, false], [true, true, true, false, false, true, true, true], [true, true, true, false, true, false, false, false], [true, true, true, false, true, false, false, true], [true, true, true, false, true, false, true, false], [true, true, true, false, true, false, true, true], [true, true, true, false, true, true, false, false], [true, true, true, false, true, true, false, true], [true, true, true, false, true, true, true, false], [true, true, true, false, true, true, true, true], [true, true, true, true, false, false, false, false], [true, true, true, true, false, false, false, true], [true, true, true, true, false, false, true, false], [true, true, true, true, false, false, true, true], [true, true, true, true, false, true, false, false], [true, true, true, true, false, true, false, true], [true, true, true, true, false, true, true, false], [true, true, true, true, false, true, true, true], [true, true, true, true, true, false, false, false], [true, true, true, true, true, false, false, true], [true, true, true, true, true, false, true, false], [true, true, true, true, true, false, true, true], [true, true, true, true, true, true, false, false], [true, true, true, true, true, true, false, true], [true, true, true, true, true, true, true, false], [true, true, true, true, true, true, true, true], ];


#[test]
fn test_u8_to_array_bool() {
    for ind in 0..=u8::MAX {
        assert_eq!(ind, _bools_to_u8(u8_to_array_bool(ind)))
    }
}

fn decode_rice(img: &[u8], remainder_bits: u8, size: usize) -> Vec<u8> {
    let mut img_code_u8 = Vec::with_capacity(size);
    let mut num = 0_u8;
    let mut remainder_index = 0_u8;
    let mut code_unary = true;

    // The algorithm can be simplified if remainder_bits == 0
    if remainder_bits == 0 {
        for &chunk in img.iter() {
            for &bit in u8_to_array_bool(chunk) {
                match bit {
                    true => num += 1,
                    false => {
                        img_code_u8.push(num);
                        num = 0;
                    }
                }
            }
        }
    } else {  // General algorithm for remainder_bits >= 1
        for chunk in img {
            for &bit in u8_to_array_bool(*chunk) {
                match code_unary {
                    true => {
                        match bit {
                            true => num += 1,
                            false => code_unary = false,
                        }
                    }
                    false => {
                        num = (num << 1) + (bit as u8);
                        remainder_index += 1;
                        if remainder_index == remainder_bits {
                            img_code_u8.push(num);
                            num = 0;
                            remainder_index = 0;
                            code_unary = true;
                        }
                    }
                }
            }
        }
    }

    img_code_u8
}

#[test]
fn test_decode_rice() {
    // Testing simple case without remainder
    assert_eq!(decode_rice(&[0], 0, 1), vec![0; 8]);
    assert_eq!(decode_rice(&[254], 0, 1), vec![7]);

    // Testing simple case with remainder
    assert_eq!(decode_rice(&[0], 3, 1), vec![0, 0]);
    // 186 == "1011 1010" => [(1 << 2) + 3, (1 << 2) + 2)]
    assert_eq!(decode_rice(&[186], 2, 1), vec![7, 6]);

    // Testing case with multiple "true" at the end, that won't be parsed
    // 127_u8 -> [false , true, ..., true; 8] -(decoded with 0 remainder bits)-> [0]
    assert_eq!(decode_rice(&[127], 0, 1), vec![0]);
}

fn _bools_to_u8(bools: &[bool; 8]) -> u8 {
    bools.iter().fold(0_u8, |num, &bool| (num << 1) + (bool as u8))
}

#[test]
fn test_bools_to_u8() {
    assert_eq!(0, _bools_to_u8(&[false, false, false, false, false, false, false, false]));
    assert_eq!(1, _bools_to_u8(&[false, false, false, false, false, false, false, true]));
    assert_eq!(255, _bools_to_u8(&[true, true, true, true, true, true, true, true]));
}

struct Header {
    _name: String,
    width: u32,
    height: u32,
    pae: u32,
    remainder: u8,
    start_index: usize,
}

/// Parses the header from the beginning of the file or from CLI `args`
fn parse_header(img: &[u8], args: &Args) -> Header {
    if args.no_header {
        // Image has no header. Using CLI arguments
        trace!("Image has no header. Using CLI arguments");
        Header {
            _name: "TGIF".to_string(),
            width: args.width.expect("Image width is missing"),
            height: args.height.expect("Image height is missing"),
            pae: args.parallel_encoding_units.expect("Number of parallel encoding units is missing"),
            remainder: args.remainder_bits.expect("Bit size of remainder is missing"),
            start_index: 0,  // Without header the data starts with the first bit
        }
    } else {
        trace!("Reading header from image");
        Header {
            _name: "TGIF".to_string(),
            width: img[4..8].iter().fold(0_u32, |res, val| (res << 8) + (*val as u32)),
            height: img[8..12].iter().fold(0_u32, |res, val| (res << 8) + (*val as u32)),
            pae: img[12..16].iter().fold(0_u32, |res, val| (res << 8) + (*val as u32)),
            remainder: img[16],
            start_index: 17,
        }
    }
}