use log::trace;

pub const STARTING_INDEX: usize = 17;

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub chunk_size: u32,
    pub rem_bits: u8,
}

impl Header {
    pub fn new(width: u32, height: u32, chunk_size: u32, rem_bits: u8) -> Self {
        Header {
            name: "TGIF".to_string(),
            height,
            width,
            chunk_size,
            rem_bits,
        }
    }

    pub fn to_u8(&self) -> Vec<u8> {
        [
            u32::from_be_bytes(*b"TGIF"),
            self.height,
            self.width,
            self.chunk_size,
        ]
        .into_iter()
        .flat_map(|v| v.to_be_bytes())
        .chain(std::iter::once(self.rem_bits))
        .collect()
    }

    pub fn from_u8(img: &[u8]) -> Self {
        trace!("Reading header from image");
        Header {
            name: "TGIF".to_string(),
            height: Self::slice_u8_as_u32_be(&img[4..8]),
            width: Self::slice_u8_as_u32_be(&img[8..12]),
            chunk_size: Self::slice_u8_as_u32_be(&img[12..16]),
            rem_bits: img[16],
        }
    }

    fn slice_u8_as_u32_be(array: &[u8]) -> u32 {
        debug_assert_eq!(array.len(), 4);
        array
            .iter()
            .fold(0_u32, |res, val| (res << 8) + (*val as u32))
    }
}
