pub const CHUNK_SIZE: usize = 1024 * 8 * 32;

pub const POW_OF_TWO: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

pub const U8_TO_ARRAY_BOOL: [[bool; 8]; 256] = [[false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, true], [false, false, false, false, false, false, true, false], [false, false, false, false, false, false, true, true], [false, false, false, false, false, true, false, false], [false, false, false, false, false, true, false, true], [false, false, false, false, false, true, true, false], [false, false, false, false, false, true, true, true], [false, false, false, false, true, false, false, false], [false, false, false, false, true, false, false, true], [false, false, false, false, true, false, true, false], [false, false, false, false, true, false, true, true], [false, false, false, false, true, true, false, false], [false, false, false, false, true, true, false, true], [false, false, false, false, true, true, true, false], [false, false, false, false, true, true, true, true], [false, false, false, true, false, false, false, false], [false, false, false, true, false, false, false, true], [false, false, false, true, false, false, true, false], [false, false, false, true, false, false, true, true], [false, false, false, true, false, true, false, false], [false, false, false, true, false, true, false, true], [false, false, false, true, false, true, true, false], [false, false, false, true, false, true, true, true], [false, false, false, true, true, false, false, false], [false, false, false, true, true, false, false, true], [false, false, false, true, true, false, true, false], [false, false, false, true, true, false, true, true], [false, false, false, true, true, true, false, false], [false, false, false, true, true, true, false, true], [false, false, false, true, true, true, true, false], [false, false, false, true, true, true, true, true], [false, false, true, false, false, false, false, false], [false, false, true, false, false, false, false, true], [false, false, true, false, false, false, true, false], [false, false, true, false, false, false, true, true], [false, false, true, false, false, true, false, false], [false, false, true, false, false, true, false, true], [false, false, true, false, false, true, true, false], [false, false, true, false, false, true, true, true], [false, false, true, false, true, false, false, false], [false, false, true, false, true, false, false, true], [false, false, true, false, true, false, true, false], [false, false, true, false, true, false, true, true], [false, false, true, false, true, true, false, false], [false, false, true, false, true, true, false, true], [false, false, true, false, true, true, true, false], [false, false, true, false, true, true, true, true], [false, false, true, true, false, false, false, false], [false, false, true, true, false, false, false, true], [false, false, true, true, false, false, true, false], [false, false, true, true, false, false, true, true], [false, false, true, true, false, true, false, false], [false, false, true, true, false, true, false, true], [false, false, true, true, false, true, true, false], [false, false, true, true, false, true, true, true], [false, false, true, true, true, false, false, false], [false, false, true, true, true, false, false, true], [false, false, true, true, true, false, true, false], [false, false, true, true, true, false, true, true], [false, false, true, true, true, true, false, false], [false, false, true, true, true, true, false, true], [false, false, true, true, true, true, true, false], [false, false, true, true, true, true, true, true], [false, true, false, false, false, false, false, false], [false, true, false, false, false, false, false, true], [false, true, false, false, false, false, true, false], [false, true, false, false, false, false, true, true], [false, true, false, false, false, true, false, false], [false, true, false, false, false, true, false, true], [false, true, false, false, false, true, true, false], [false, true, false, false, false, true, true, true], [false, true, false, false, true, false, false, false], [false, true, false, false, true, false, false, true], [false, true, false, false, true, false, true, false], [false, true, false, false, true, false, true, true], [false, true, false, false, true, true, false, false], [false, true, false, false, true, true, false, true], [false, true, false, false, true, true, true, false], [false, true, false, false, true, true, true, true], [false, true, false, true, false, false, false, false], [false, true, false, true, false, false, false, true], [false, true, false, true, false, false, true, false], [false, true, false, true, false, false, true, true], [false, true, false, true, false, true, false, false], [false, true, false, true, false, true, false, true], [false, true, false, true, false, true, true, false], [false, true, false, true, false, true, true, true], [false, true, false, true, true, false, false, false], [false, true, false, true, true, false, false, true], [false, true, false, true, true, false, true, false], [false, true, false, true, true, false, true, true], [false, true, false, true, true, true, false, false], [false, true, false, true, true, true, false, true], [false, true, false, true, true, true, true, false], [false, true, false, true, true, true, true, true], [false, true, true, false, false, false, false, false], [false, true, true, false, false, false, false, true], [false, true, true, false, false, false, true, false], [false, true, true, false, false, false, true, true], [false, true, true, false, false, true, false, false], [false, true, true, false, false, true, false, true], [false, true, true, false, false, true, true, false], [false, true, true, false, false, true, true, true], [false, true, true, false, true, false, false, false], [false, true, true, false, true, false, false, true], [false, true, true, false, true, false, true, false], [false, true, true, false, true, false, true, true], [false, true, true, false, true, true, false, false], [false, true, true, false, true, true, false, true], [false, true, true, false, true, true, true, false], [false, true, true, false, true, true, true, true], [false, true, true, true, false, false, false, false], [false, true, true, true, false, false, false, true], [false, true, true, true, false, false, true, false], [false, true, true, true, false, false, true, true], [false, true, true, true, false, true, false, false], [false, true, true, true, false, true, false, true], [false, true, true, true, false, true, true, false], [false, true, true, true, false, true, true, true], [false, true, true, true, true, false, false, false], [false, true, true, true, true, false, false, true], [false, true, true, true, true, false, true, false], [false, true, true, true, true, false, true, true], [false, true, true, true, true, true, false, false], [false, true, true, true, true, true, false, true], [false, true, true, true, true, true, true, false], [false, true, true, true, true, true, true, true], [true, false, false, false, false, false, false, false], [true, false, false, false, false, false, false, true], [true, false, false, false, false, false, true, false], [true, false, false, false, false, false, true, true], [true, false, false, false, false, true, false, false], [true, false, false, false, false, true, false, true], [true, false, false, false, false, true, true, false], [true, false, false, false, false, true, true, true], [true, false, false, false, true, false, false, false], [true, false, false, false, true, false, false, true], [true, false, false, false, true, false, true, false], [true, false, false, false, true, false, true, true], [true, false, false, false, true, true, false, false], [true, false, false, false, true, true, false, true], [true, false, false, false, true, true, true, false], [true, false, false, false, true, true, true, true], [true, false, false, true, false, false, false, false], [true, false, false, true, false, false, false, true], [true, false, false, true, false, false, true, false], [true, false, false, true, false, false, true, true], [true, false, false, true, false, true, false, false], [true, false, false, true, false, true, false, true], [true, false, false, true, false, true, true, false], [true, false, false, true, false, true, true, true], [true, false, false, true, true, false, false, false], [true, false, false, true, true, false, false, true], [true, false, false, true, true, false, true, false], [true, false, false, true, true, false, true, true], [true, false, false, true, true, true, false, false], [true, false, false, true, true, true, false, true], [true, false, false, true, true, true, true, false], [true, false, false, true, true, true, true, true], [true, false, true, false, false, false, false, false], [true, false, true, false, false, false, false, true], [true, false, true, false, false, false, true, false], [true, false, true, false, false, false, true, true], [true, false, true, false, false, true, false, false], [true, false, true, false, false, true, false, true], [true, false, true, false, false, true, true, false], [true, false, true, false, false, true, true, true], [true, false, true, false, true, false, false, false], [true, false, true, false, true, false, false, true], [true, false, true, false, true, false, true, false], [true, false, true, false, true, false, true, true], [true, false, true, false, true, true, false, false], [true, false, true, false, true, true, false, true], [true, false, true, false, true, true, true, false], [true, false, true, false, true, true, true, true], [true, false, true, true, false, false, false, false], [true, false, true, true, false, false, false, true], [true, false, true, true, false, false, true, false], [true, false, true, true, false, false, true, true], [true, false, true, true, false, true, false, false], [true, false, true, true, false, true, false, true], [true, false, true, true, false, true, true, false], [true, false, true, true, false, true, true, true], [true, false, true, true, true, false, false, false], [true, false, true, true, true, false, false, true], [true, false, true, true, true, false, true, false], [true, false, true, true, true, false, true, true], [true, false, true, true, true, true, false, false], [true, false, true, true, true, true, false, true], [true, false, true, true, true, true, true, false], [true, false, true, true, true, true, true, true], [true, true, false, false, false, false, false, false], [true, true, false, false, false, false, false, true], [true, true, false, false, false, false, true, false], [true, true, false, false, false, false, true, true], [true, true, false, false, false, true, false, false], [true, true, false, false, false, true, false, true], [true, true, false, false, false, true, true, false], [true, true, false, false, false, true, true, true], [true, true, false, false, true, false, false, false], [true, true, false, false, true, false, false, true], [true, true, false, false, true, false, true, false], [true, true, false, false, true, false, true, true], [true, true, false, false, true, true, false, false], [true, true, false, false, true, true, false, true], [true, true, false, false, true, true, true, false], [true, true, false, false, true, true, true, true], [true, true, false, true, false, false, false, false], [true, true, false, true, false, false, false, true], [true, true, false, true, false, false, true, false], [true, true, false, true, false, false, true, true], [true, true, false, true, false, true, false, false], [true, true, false, true, false, true, false, true], [true, true, false, true, false, true, true, false], [true, true, false, true, false, true, true, true], [true, true, false, true, true, false, false, false], [true, true, false, true, true, false, false, true], [true, true, false, true, true, false, true, false], [true, true, false, true, true, false, true, true], [true, true, false, true, true, true, false, false], [true, true, false, true, true, true, false, true], [true, true, false, true, true, true, true, false], [true, true, false, true, true, true, true, true], [true, true, true, false, false, false, false, false], [true, true, true, false, false, false, false, true], [true, true, true, false, false, false, true, false], [true, true, true, false, false, false, true, true], [true, true, true, false, false, true, false, false], [true, true, true, false, false, true, false, true], [true, true, true, false, false, true, true, false], [true, true, true, false, false, true, true, true], [true, true, true, false, true, false, false, false], [true, true, true, false, true, false, false, true], [true, true, true, false, true, false, true, false], [true, true, true, false, true, false, true, true], [true, true, true, false, true, true, false, false], [true, true, true, false, true, true, false, true], [true, true, true, false, true, true, true, false], [true, true, true, false, true, true, true, true], [true, true, true, true, false, false, false, false], [true, true, true, true, false, false, false, true], [true, true, true, true, false, false, true, false], [true, true, true, true, false, false, true, true], [true, true, true, true, false, true, false, false], [true, true, true, true, false, true, false, true], [true, true, true, true, false, true, true, false], [true, true, true, true, false, true, true, true], [true, true, true, true, true, false, false, false], [true, true, true, true, true, false, false, true], [true, true, true, true, true, false, true, false], [true, true, true, true, true, false, true, true], [true, true, true, true, true, true, false, false], [true, true, true, true, true, true, false, true], [true, true, true, true, true, true, true, false], [true, true, true, true, true, true, true, true], ];

pub const REV_RICE_INDEX: [u8; 256] = [0, 255, 1, 254, 2, 253, 3, 252, 4, 251, 5, 250, 6, 249, 7, 248, 8, 247, 9, 246, 10, 245, 11, 244, 12, 243, 13, 242, 14, 241, 15, 240, 16, 239, 17, 238, 18, 237, 19, 236, 20, 235, 21, 234, 22, 233, 23, 232, 24, 231, 25, 230, 26, 229, 27, 228, 28, 227, 29, 226, 30, 225, 31, 224, 32, 223, 33, 222, 34, 221, 35, 220, 36, 219, 37, 218, 38, 217, 39, 216, 40, 215, 41, 214, 42, 213, 43, 212, 44, 211, 45, 210, 46, 209, 47, 208, 48, 207, 49, 206, 50, 205, 51, 204, 52, 203, 53, 202, 54, 201, 55, 200, 56, 199, 57, 198, 58, 197, 59, 196, 60, 195, 61, 194, 62, 193, 63, 192, 64, 191, 65, 190, 66, 189, 67, 188, 68, 187, 69, 186, 70, 185, 71, 184, 72, 183, 73, 182, 74, 181, 75, 180, 76, 179, 77, 178, 78, 177, 79, 176, 80, 175, 81, 174, 82, 173, 83, 172, 84, 171, 85, 170, 86, 169, 87, 168, 88, 167, 89, 166, 90, 165, 91, 164, 92, 163, 93, 162, 94, 161, 95, 160, 96, 159, 97, 158, 98, 157, 99, 156, 100, 155, 101, 154, 102, 153, 103, 152, 104, 151, 105, 150, 106, 149, 107, 148, 108, 147, 109, 146, 110, 145, 111, 144, 112, 143, 113, 142, 114, 141, 115, 140, 116, 139, 117, 138, 118, 137, 119, 136, 120, 135, 121, 134, 122, 133, 123, 132, 124, 131, 125, 130, 126, 129, 127, 128];


#[allow(clippy::assertions_on_constants, unused_imports)]
pub mod tests {
    use super::*;
    #[test]
    fn test_constants() {
        assert!(CHUNK_SIZE >= 257);  // The biggest indivisible chunk is 256 * "1" + "0"
        assert_eq!(CHUNK_SIZE % 8, 0);  // Chunks of bits must divisible in bytes
    }

}