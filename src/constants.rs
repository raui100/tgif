pub const POW_OF_TWO: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

pub const RICE_INDEX: [u8; 256] = [
    0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48,
    50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96,
    98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126, 128, 130, 132, 134,
    136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 166, 168, 170, 172,
    174, 176, 178, 180, 182, 184, 186, 188, 190, 192, 194, 196, 198, 200, 202, 204, 206, 208, 210,
    212, 214, 216, 218, 220, 222, 224, 226, 228, 230, 232, 234, 236, 238, 240, 242, 244, 246, 248,
    250, 252, 254, 255, 253, 251, 249, 247, 245, 243, 241, 239, 237, 235, 233, 231, 229, 227, 225,
    223, 221, 219, 217, 215, 213, 211, 209, 207, 205, 203, 201, 199, 197, 195, 193, 191, 189, 187,
    185, 183, 181, 179, 177, 175, 173, 171, 169, 167, 165, 163, 161, 159, 157, 155, 153, 151, 149,
    147, 145, 143, 141, 139, 137, 135, 133, 131, 129, 127, 125, 123, 121, 119, 117, 115, 113, 111,
    109, 107, 105, 103, 101, 99, 97, 95, 93, 91, 89, 87, 85, 83, 81, 79, 77, 75, 73, 71, 69, 67,
    65, 63, 61, 59, 57, 55, 53, 51, 49, 47, 45, 43, 41, 39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 19,
    17, 15, 13, 11, 9, 7, 5, 3, 1,
];

pub const REV_RICE_INDEX: [u8; 256] = [
    0, 255, 1, 254, 2, 253, 3, 252, 4, 251, 5, 250, 6, 249, 7, 248, 8, 247, 9, 246, 10, 245, 11,
    244, 12, 243, 13, 242, 14, 241, 15, 240, 16, 239, 17, 238, 18, 237, 19, 236, 20, 235, 21, 234,
    22, 233, 23, 232, 24, 231, 25, 230, 26, 229, 27, 228, 28, 227, 29, 226, 30, 225, 31, 224, 32,
    223, 33, 222, 34, 221, 35, 220, 36, 219, 37, 218, 38, 217, 39, 216, 40, 215, 41, 214, 42, 213,
    43, 212, 44, 211, 45, 210, 46, 209, 47, 208, 48, 207, 49, 206, 50, 205, 51, 204, 52, 203, 53,
    202, 54, 201, 55, 200, 56, 199, 57, 198, 58, 197, 59, 196, 60, 195, 61, 194, 62, 193, 63, 192,
    64, 191, 65, 190, 66, 189, 67, 188, 68, 187, 69, 186, 70, 185, 71, 184, 72, 183, 73, 182, 74,
    181, 75, 180, 76, 179, 77, 178, 78, 177, 79, 176, 80, 175, 81, 174, 82, 173, 83, 172, 84, 171,
    85, 170, 86, 169, 87, 168, 88, 167, 89, 166, 90, 165, 91, 164, 92, 163, 93, 162, 94, 161, 95,
    160, 96, 159, 97, 158, 98, 157, 99, 156, 100, 155, 101, 154, 102, 153, 103, 152, 104, 151, 105,
    150, 106, 149, 107, 148, 108, 147, 109, 146, 110, 145, 111, 144, 112, 143, 113, 142, 114, 141,
    115, 140, 116, 139, 117, 138, 118, 137, 119, 136, 120, 135, 121, 134, 122, 133, 123, 132, 124,
    131, 125, 130, 126, 129, 127, 128,
];

pub const U8_TO_ARRAY_BOOL: [[u8; 8]; 256] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 1, 0, 1],
    [0, 0, 0, 0, 0, 1, 1, 0],
    [0, 0, 0, 0, 0, 1, 1, 1],
    [0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 1],
    [0, 0, 0, 0, 1, 0, 1, 0],
    [0, 0, 0, 0, 1, 0, 1, 1],
    [0, 0, 0, 0, 1, 1, 0, 0],
    [0, 0, 0, 0, 1, 1, 0, 1],
    [0, 0, 0, 0, 1, 1, 1, 0],
    [0, 0, 0, 0, 1, 1, 1, 1],
    [0, 0, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 1],
    [0, 0, 0, 1, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0, 1, 1],
    [0, 0, 0, 1, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 1, 0, 1],
    [0, 0, 0, 1, 0, 1, 1, 0],
    [0, 0, 0, 1, 0, 1, 1, 1],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 1],
    [0, 0, 0, 1, 1, 0, 1, 0],
    [0, 0, 0, 1, 1, 0, 1, 1],
    [0, 0, 0, 1, 1, 1, 0, 0],
    [0, 0, 0, 1, 1, 1, 0, 1],
    [0, 0, 0, 1, 1, 1, 1, 0],
    [0, 0, 0, 1, 1, 1, 1, 1],
    [0, 0, 1, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 1],
    [0, 0, 1, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 0, 1, 1],
    [0, 0, 1, 0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0, 1, 0, 1],
    [0, 0, 1, 0, 0, 1, 1, 0],
    [0, 0, 1, 0, 0, 1, 1, 1],
    [0, 0, 1, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 1, 0, 0, 1],
    [0, 0, 1, 0, 1, 0, 1, 0],
    [0, 0, 1, 0, 1, 0, 1, 1],
    [0, 0, 1, 0, 1, 1, 0, 0],
    [0, 0, 1, 0, 1, 1, 0, 1],
    [0, 0, 1, 0, 1, 1, 1, 0],
    [0, 0, 1, 0, 1, 1, 1, 1],
    [0, 0, 1, 1, 0, 0, 0, 0],
    [0, 0, 1, 1, 0, 0, 0, 1],
    [0, 0, 1, 1, 0, 0, 1, 0],
    [0, 0, 1, 1, 0, 0, 1, 1],
    [0, 0, 1, 1, 0, 1, 0, 0],
    [0, 0, 1, 1, 0, 1, 0, 1],
    [0, 0, 1, 1, 0, 1, 1, 0],
    [0, 0, 1, 1, 0, 1, 1, 1],
    [0, 0, 1, 1, 1, 0, 0, 0],
    [0, 0, 1, 1, 1, 0, 0, 1],
    [0, 0, 1, 1, 1, 0, 1, 0],
    [0, 0, 1, 1, 1, 0, 1, 1],
    [0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 1],
    [0, 0, 1, 1, 1, 1, 1, 0],
    [0, 0, 1, 1, 1, 1, 1, 1],
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 1, 1],
    [0, 1, 0, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0, 1, 0, 1],
    [0, 1, 0, 0, 0, 1, 1, 0],
    [0, 1, 0, 0, 0, 1, 1, 1],
    [0, 1, 0, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 1, 0, 0, 1],
    [0, 1, 0, 0, 1, 0, 1, 0],
    [0, 1, 0, 0, 1, 0, 1, 1],
    [0, 1, 0, 0, 1, 1, 0, 0],
    [0, 1, 0, 0, 1, 1, 0, 1],
    [0, 1, 0, 0, 1, 1, 1, 0],
    [0, 1, 0, 0, 1, 1, 1, 1],
    [0, 1, 0, 1, 0, 0, 0, 0],
    [0, 1, 0, 1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0, 0, 1, 0],
    [0, 1, 0, 1, 0, 0, 1, 1],
    [0, 1, 0, 1, 0, 1, 0, 0],
    [0, 1, 0, 1, 0, 1, 0, 1],
    [0, 1, 0, 1, 0, 1, 1, 0],
    [0, 1, 0, 1, 0, 1, 1, 1],
    [0, 1, 0, 1, 1, 0, 0, 0],
    [0, 1, 0, 1, 1, 0, 0, 1],
    [0, 1, 0, 1, 1, 0, 1, 0],
    [0, 1, 0, 1, 1, 0, 1, 1],
    [0, 1, 0, 1, 1, 1, 0, 0],
    [0, 1, 0, 1, 1, 1, 0, 1],
    [0, 1, 0, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 1, 1, 1, 1],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 1],
    [0, 1, 1, 0, 0, 0, 1, 0],
    [0, 1, 1, 0, 0, 0, 1, 1],
    [0, 1, 1, 0, 0, 1, 0, 0],
    [0, 1, 1, 0, 0, 1, 0, 1],
    [0, 1, 1, 0, 0, 1, 1, 0],
    [0, 1, 1, 0, 0, 1, 1, 1],
    [0, 1, 1, 0, 1, 0, 0, 0],
    [0, 1, 1, 0, 1, 0, 0, 1],
    [0, 1, 1, 0, 1, 0, 1, 0],
    [0, 1, 1, 0, 1, 0, 1, 1],
    [0, 1, 1, 0, 1, 1, 0, 0],
    [0, 1, 1, 0, 1, 1, 0, 1],
    [0, 1, 1, 0, 1, 1, 1, 0],
    [0, 1, 1, 0, 1, 1, 1, 1],
    [0, 1, 1, 1, 0, 0, 0, 0],
    [0, 1, 1, 1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0, 0, 1, 0],
    [0, 1, 1, 1, 0, 0, 1, 1],
    [0, 1, 1, 1, 0, 1, 0, 0],
    [0, 1, 1, 1, 0, 1, 0, 1],
    [0, 1, 1, 1, 0, 1, 1, 0],
    [0, 1, 1, 1, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 1],
    [0, 1, 1, 1, 1, 0, 1, 0],
    [0, 1, 1, 1, 1, 0, 1, 1],
    [0, 1, 1, 1, 1, 1, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 0, 1, 0, 0],
    [1, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 0],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [1, 0, 0, 0, 1, 0, 0, 0],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1, 0, 1, 1],
    [1, 0, 0, 0, 1, 1, 0, 0],
    [1, 0, 0, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1, 1, 1, 1],
    [1, 0, 0, 1, 0, 0, 0, 0],
    [1, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 1, 0],
    [1, 0, 0, 1, 0, 0, 1, 1],
    [1, 0, 0, 1, 0, 1, 0, 0],
    [1, 0, 0, 1, 0, 1, 0, 1],
    [1, 0, 0, 1, 0, 1, 1, 0],
    [1, 0, 0, 1, 0, 1, 1, 1],
    [1, 0, 0, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 0, 0, 1],
    [1, 0, 0, 1, 1, 0, 1, 0],
    [1, 0, 0, 1, 1, 0, 1, 1],
    [1, 0, 0, 1, 1, 1, 0, 0],
    [1, 0, 0, 1, 1, 1, 0, 1],
    [1, 0, 0, 1, 1, 1, 1, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 0],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 1, 0],
    [1, 0, 1, 0, 0, 0, 1, 1],
    [1, 0, 1, 0, 0, 1, 0, 0],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 1, 1, 0],
    [1, 0, 1, 0, 0, 1, 1, 1],
    [1, 0, 1, 0, 1, 0, 0, 0],
    [1, 0, 1, 0, 1, 0, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 0],
    [1, 0, 1, 0, 1, 0, 1, 1],
    [1, 0, 1, 0, 1, 1, 0, 0],
    [1, 0, 1, 0, 1, 1, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 0],
    [1, 0, 1, 0, 1, 1, 1, 1],
    [1, 0, 1, 1, 0, 0, 0, 0],
    [1, 0, 1, 1, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 0, 1, 0],
    [1, 0, 1, 1, 0, 0, 1, 1],
    [1, 0, 1, 1, 0, 1, 0, 0],
    [1, 0, 1, 1, 0, 1, 0, 1],
    [1, 0, 1, 1, 0, 1, 1, 0],
    [1, 0, 1, 1, 0, 1, 1, 1],
    [1, 0, 1, 1, 1, 0, 0, 0],
    [1, 0, 1, 1, 1, 0, 0, 1],
    [1, 0, 1, 1, 1, 0, 1, 0],
    [1, 0, 1, 1, 1, 0, 1, 1],
    [1, 0, 1, 1, 1, 1, 0, 0],
    [1, 0, 1, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 0],
    [1, 0, 1, 1, 1, 1, 1, 1],
    [1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0, 0, 1],
    [1, 1, 0, 0, 0, 0, 1, 0],
    [1, 1, 0, 0, 0, 0, 1, 1],
    [1, 1, 0, 0, 0, 1, 0, 0],
    [1, 1, 0, 0, 0, 1, 0, 1],
    [1, 1, 0, 0, 0, 1, 1, 0],
    [1, 1, 0, 0, 0, 1, 1, 1],
    [1, 1, 0, 0, 1, 0, 0, 0],
    [1, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 0, 0, 1, 0, 1, 0],
    [1, 1, 0, 0, 1, 0, 1, 1],
    [1, 1, 0, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 0, 1],
    [1, 1, 0, 0, 1, 1, 1, 0],
    [1, 1, 0, 0, 1, 1, 1, 1],
    [1, 1, 0, 1, 0, 0, 0, 0],
    [1, 1, 0, 1, 0, 0, 0, 1],
    [1, 1, 0, 1, 0, 0, 1, 0],
    [1, 1, 0, 1, 0, 0, 1, 1],
    [1, 1, 0, 1, 0, 1, 0, 0],
    [1, 1, 0, 1, 0, 1, 0, 1],
    [1, 1, 0, 1, 0, 1, 1, 0],
    [1, 1, 0, 1, 0, 1, 1, 1],
    [1, 1, 0, 1, 1, 0, 0, 0],
    [1, 1, 0, 1, 1, 0, 0, 1],
    [1, 1, 0, 1, 1, 0, 1, 0],
    [1, 1, 0, 1, 1, 0, 1, 1],
    [1, 1, 0, 1, 1, 1, 0, 0],
    [1, 1, 0, 1, 1, 1, 0, 1],
    [1, 1, 0, 1, 1, 1, 1, 0],
    [1, 1, 0, 1, 1, 1, 1, 1],
    [1, 1, 1, 0, 0, 0, 0, 0],
    [1, 1, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 0, 0, 0, 1, 0],
    [1, 1, 1, 0, 0, 0, 1, 1],
    [1, 1, 1, 0, 0, 1, 0, 0],
    [1, 1, 1, 0, 0, 1, 0, 1],
    [1, 1, 1, 0, 0, 1, 1, 0],
    [1, 1, 1, 0, 0, 1, 1, 1],
    [1, 1, 1, 0, 1, 0, 0, 0],
    [1, 1, 1, 0, 1, 0, 0, 1],
    [1, 1, 1, 0, 1, 0, 1, 0],
    [1, 1, 1, 0, 1, 0, 1, 1],
    [1, 1, 1, 0, 1, 1, 0, 0],
    [1, 1, 1, 0, 1, 1, 0, 1],
    [1, 1, 1, 0, 1, 1, 1, 0],
    [1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0, 0, 1, 0],
    [1, 1, 1, 1, 0, 0, 1, 1],
    [1, 1, 1, 1, 0, 1, 0, 0],
    [1, 1, 1, 1, 0, 1, 0, 1],
    [1, 1, 1, 1, 0, 1, 1, 0],
    [1, 1, 1, 1, 0, 1, 1, 1],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 0, 1],
    [1, 1, 1, 1, 1, 0, 1, 0],
    [1, 1, 1, 1, 1, 0, 1, 1],
    [1, 1, 1, 1, 1, 1, 0, 0],
    [1, 1, 1, 1, 1, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 0],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// Calculates the rice index for a given number
    fn rice_index(num: u8) -> u8 {
        if num <= 127 {
            num * 2
        } else {
            (u8::MAX - num) * 2 + 1
        }
    }

    #[test]
    fn test_rice_index() {
        for num in 0..=u8::MAX {
            assert_eq!(rice_index(num), RICE_INDEX[num as usize]);
        }
    }
}
