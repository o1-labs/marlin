/*

This source file Rust-adapts the following optimized AES implementation:
    "byte-oriented-aes – A public domain byte-oriented implementation of AES in C – Google Project Hosting".
    Archived from the original on 2013-07-20. Retrieved 2012-12-23.
    https://code.google.com/p/byte-oriented-aes
*/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use rand::{thread_rng, Rng};
use std::convert::TryInto;

// forward s-box
const Sbox: [u8; 256] =
[
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

// inverse s-box
const InvSbox: [u8; 256] =
[
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d
];
    
// combined Xtimes2[Sbox[]]
const Xtime2Sbox: [u8; 256] =
[
    0xc6, 0xf8, 0xee, 0xf6, 0xff, 0xd6, 0xde, 0x91, 0x60, 0x02, 0xce, 0x56, 0xe7, 0xb5, 0x4d, 0xec, 
    0x8f, 0x1f, 0x89, 0xfa, 0xef, 0xb2, 0x8e, 0xfb, 0x41, 0xb3, 0x5f, 0x45, 0x23, 0x53, 0xe4, 0x9b, 
    0x75, 0xe1, 0x3d, 0x4c, 0x6c, 0x7e, 0xf5, 0x83, 0x68, 0x51, 0xd1, 0xf9, 0xe2, 0xab, 0x62, 0x2a, 
    0x08, 0x95, 0x46, 0x9d, 0x30, 0x37, 0x0a, 0x2f, 0x0e, 0x24, 0x1b, 0xdf, 0xcd, 0x4e, 0x7f, 0xea, 
    0x12, 0x1d, 0x58, 0x34, 0x36, 0xdc, 0xb4, 0x5b, 0xa4, 0x76, 0xb7, 0x7d, 0x52, 0xdd, 0x5e, 0x13, 
    0xa6, 0xb9, 0x00, 0xc1, 0x40, 0xe3, 0x79, 0xb6, 0xd4, 0x8d, 0x67, 0x72, 0x94, 0x98, 0xb0, 0x85, 
    0xbb, 0xc5, 0x4f, 0xed, 0x86, 0x9a, 0x66, 0x11, 0x8a, 0xe9, 0x04, 0xfe, 0xa0, 0x78, 0x25, 0x4b, 
    0xa2, 0x5d, 0x80, 0x05, 0x3f, 0x21, 0x70, 0xf1, 0x63, 0x77, 0xaf, 0x42, 0x20, 0xe5, 0xfd, 0xbf, 
    0x81, 0x18, 0x26, 0xc3, 0xbe, 0x35, 0x88, 0x2e, 0x93, 0x55, 0xfc, 0x7a, 0xc8, 0xba, 0x32, 0xe6, 
    0xc0, 0x19, 0x9e, 0xa3, 0x44, 0x54, 0x3b, 0x0b, 0x8c, 0xc7, 0x6b, 0x28, 0xa7, 0xbc, 0x16, 0xad, 
    0xdb, 0x64, 0x74, 0x14, 0x92, 0x0c, 0x48, 0xb8, 0x9f, 0xbd, 0x43, 0xc4, 0x39, 0x31, 0xd3, 0xf2, 
    0xd5, 0x8b, 0x6e, 0xda, 0x01, 0xb1, 0x9c, 0x49, 0xd8, 0xac, 0xf3, 0xcf, 0xca, 0xf4, 0x47, 0x10, 
    0x6f, 0xf0, 0x4a, 0x5c, 0x38, 0x57, 0x73, 0x97, 0xcb, 0xa1, 0xe8, 0x3e, 0x96, 0x61, 0x0d, 0x0f, 
    0xe0, 0x7c, 0x71, 0xcc, 0x90, 0x06, 0xf7, 0x1c, 0xc2, 0x6a, 0xae, 0x69, 0x17, 0x99, 0x3a, 0x27, 
    0xd9, 0xeb, 0x2b, 0x22, 0xd2, 0xa9, 0x07, 0x33, 0x2d, 0x3c, 0x15, 0xc9, 0x87, 0xaa, 0x50, 0xa5, 
    0x03, 0x59, 0x09, 0x1a, 0x65, 0xd7, 0x84, 0xd0, 0x82, 0x29, 0x5a, 0x1e, 0x7b, 0xa8, 0x6d, 0x2c 
];
    
// combined Xtimes3[Sbox[]]
const Xtime3Sbox: [u8; 256] =
[
    0xa5, 0x84, 0x99, 0x8d, 0x0d, 0xbd, 0xb1, 0x54, 0x50, 0x03, 0xa9, 0x7d, 0x19, 0x62, 0xe6, 0x9a, 
    0x45, 0x9d, 0x40, 0x87, 0x15, 0xeb, 0xc9, 0x0b, 0xec, 0x67, 0xfd, 0xea, 0xbf, 0xf7, 0x96, 0x5b, 
    0xc2, 0x1c, 0xae, 0x6a, 0x5a, 0x41, 0x02, 0x4f, 0x5c, 0xf4, 0x34, 0x08, 0x93, 0x73, 0x53, 0x3f, 
    0x0c, 0x52, 0x65, 0x5e, 0x28, 0xa1, 0x0f, 0xb5, 0x09, 0x36, 0x9b, 0x3d, 0x26, 0x69, 0xcd, 0x9f, 
    0x1b, 0x9e, 0x74, 0x2e, 0x2d, 0xb2, 0xee, 0xfb, 0xf6, 0x4d, 0x61, 0xce, 0x7b, 0x3e, 0x71, 0x97, 
    0xf5, 0x68, 0x00, 0x2c, 0x60, 0x1f, 0xc8, 0xed, 0xbe, 0x46, 0xd9, 0x4b, 0xde, 0xd4, 0xe8, 0x4a, 
    0x6b, 0x2a, 0xe5, 0x16, 0xc5, 0xd7, 0x55, 0x94, 0xcf, 0x10, 0x06, 0x81, 0xf0, 0x44, 0xba, 0xe3, 
    0xf3, 0xfe, 0xc0, 0x8a, 0xad, 0xbc, 0x48, 0x04, 0xdf, 0xc1, 0x75, 0x63, 0x30, 0x1a, 0x0e, 0x6d, 
    0x4c, 0x14, 0x35, 0x2f, 0xe1, 0xa2, 0xcc, 0x39, 0x57, 0xf2, 0x82, 0x47, 0xac, 0xe7, 0x2b, 0x95, 
    0xa0, 0x98, 0xd1, 0x7f, 0x66, 0x7e, 0xab, 0x83, 0xca, 0x29, 0xd3, 0x3c, 0x79, 0xe2, 0x1d, 0x76, 
    0x3b, 0x56, 0x4e, 0x1e, 0xdb, 0x0a, 0x6c, 0xe4, 0x5d, 0x6e, 0xef, 0xa6, 0xa8, 0xa4, 0x37, 0x8b, 
    0x32, 0x43, 0x59, 0xb7, 0x8c, 0x64, 0xd2, 0xe0, 0xb4, 0xfa, 0x07, 0x25, 0xaf, 0x8e, 0xe9, 0x18, 
    0xd5, 0x88, 0x6f, 0x72, 0x24, 0xf1, 0xc7, 0x51, 0x23, 0x7c, 0x9c, 0x21, 0xdd, 0xdc, 0x86, 0x85, 
    0x90, 0x42, 0xc4, 0xaa, 0xd8, 0x05, 0x01, 0x12, 0xa3, 0x5f, 0xf9, 0xd0, 0x91, 0x58, 0x27, 0xb9, 
    0x38, 0x13, 0xb3, 0x33, 0xbb, 0x70, 0x89, 0xa7, 0xb6, 0x22, 0x92, 0x20, 0x49, 0xff, 0x78, 0x7a, 
    0x8f, 0xf8, 0x80, 0x17, 0xda, 0x31, 0xc6, 0xb8, 0xc3, 0xb0, 0x77, 0x11, 0xcb, 0xfc, 0xd6, 0x3a 
];
    
// modular multiplication tables
// based on:

// Xtime2[x] = (x & 0x80 ? 0x1b : 0) ^ (x + x)
// Xtime3[x] = x^Xtime2[x];

const Xtime9: [u8; 256] =
[
    0x00, 0x09, 0x12, 0x1b, 0x24, 0x2d, 0x36, 0x3f, 0x48, 0x41, 0x5a, 0x53, 0x6c, 0x65, 0x7e, 0x77, 
    0x90, 0x99, 0x82, 0x8b, 0xb4, 0xbd, 0xa6, 0xaf, 0xd8, 0xd1, 0xca, 0xc3, 0xfc, 0xf5, 0xee, 0xe7, 
    0x3b, 0x32, 0x29, 0x20, 0x1f, 0x16, 0x0d, 0x04, 0x73, 0x7a, 0x61, 0x68, 0x57, 0x5e, 0x45, 0x4c, 
    0xab, 0xa2, 0xb9, 0xb0, 0x8f, 0x86, 0x9d, 0x94, 0xe3, 0xea, 0xf1, 0xf8, 0xc7, 0xce, 0xd5, 0xdc, 
    0x76, 0x7f, 0x64, 0x6d, 0x52, 0x5b, 0x40, 0x49, 0x3e, 0x37, 0x2c, 0x25, 0x1a, 0x13, 0x08, 0x01, 
    0xe6, 0xef, 0xf4, 0xfd, 0xc2, 0xcb, 0xd0, 0xd9, 0xae, 0xa7, 0xbc, 0xb5, 0x8a, 0x83, 0x98, 0x91, 
    0x4d, 0x44, 0x5f, 0x56, 0x69, 0x60, 0x7b, 0x72, 0x05, 0x0c, 0x17, 0x1e, 0x21, 0x28, 0x33, 0x3a, 
    0xdd, 0xd4, 0xcf, 0xc6, 0xf9, 0xf0, 0xeb, 0xe2, 0x95, 0x9c, 0x87, 0x8e, 0xb1, 0xb8, 0xa3, 0xaa, 
    0xec, 0xe5, 0xfe, 0xf7, 0xc8, 0xc1, 0xda, 0xd3, 0xa4, 0xad, 0xb6, 0xbf, 0x80, 0x89, 0x92, 0x9b, 
    0x7c, 0x75, 0x6e, 0x67, 0x58, 0x51, 0x4a, 0x43, 0x34, 0x3d, 0x26, 0x2f, 0x10, 0x19, 0x02, 0x0b, 
    0xd7, 0xde, 0xc5, 0xcc, 0xf3, 0xfa, 0xe1, 0xe8, 0x9f, 0x96, 0x8d, 0x84, 0xbb, 0xb2, 0xa9, 0xa0, 
    0x47, 0x4e, 0x55, 0x5c, 0x63, 0x6a, 0x71, 0x78, 0x0f, 0x06, 0x1d, 0x14, 0x2b, 0x22, 0x39, 0x30, 
    0x9a, 0x93, 0x88, 0x81, 0xbe, 0xb7, 0xac, 0xa5, 0xd2, 0xdb, 0xc0, 0xc9, 0xf6, 0xff, 0xe4, 0xed, 
    0x0a, 0x03, 0x18, 0x11, 0x2e, 0x27, 0x3c, 0x35, 0x42, 0x4b, 0x50, 0x59, 0x66, 0x6f, 0x74, 0x7d, 
    0xa1, 0xa8, 0xb3, 0xba, 0x85, 0x8c, 0x97, 0x9e, 0xe9, 0xe0, 0xfb, 0xf2, 0xcd, 0xc4, 0xdf, 0xd6, 
    0x31, 0x38, 0x23, 0x2a, 0x15, 0x1c, 0x07, 0x0e, 0x79, 0x70, 0x6b, 0x62, 0x5d, 0x54, 0x4f, 0x46
];
    
const XtimeB: [u8; 256] =
[
    0x00, 0x0b, 0x16, 0x1d, 0x2c, 0x27, 0x3a, 0x31, 0x58, 0x53, 0x4e, 0x45, 0x74, 0x7f, 0x62, 0x69, 
    0xb0, 0xbb, 0xa6, 0xad, 0x9c, 0x97, 0x8a, 0x81, 0xe8, 0xe3, 0xfe, 0xf5, 0xc4, 0xcf, 0xd2, 0xd9, 
    0x7b, 0x70, 0x6d, 0x66, 0x57, 0x5c, 0x41, 0x4a, 0x23, 0x28, 0x35, 0x3e, 0x0f, 0x04, 0x19, 0x12, 
    0xcb, 0xc0, 0xdd, 0xd6, 0xe7, 0xec, 0xf1, 0xfa, 0x93, 0x98, 0x85, 0x8e, 0xbf, 0xb4, 0xa9, 0xa2, 
    0xf6, 0xfd, 0xe0, 0xeb, 0xda, 0xd1, 0xcc, 0xc7, 0xae, 0xa5, 0xb8, 0xb3, 0x82, 0x89, 0x94, 0x9f, 
    0x46, 0x4d, 0x50, 0x5b, 0x6a, 0x61, 0x7c, 0x77, 0x1e, 0x15, 0x08, 0x03, 0x32, 0x39, 0x24, 0x2f, 
    0x8d, 0x86, 0x9b, 0x90, 0xa1, 0xaa, 0xb7, 0xbc, 0xd5, 0xde, 0xc3, 0xc8, 0xf9, 0xf2, 0xef, 0xe4, 
    0x3d, 0x36, 0x2b, 0x20, 0x11, 0x1a, 0x07, 0x0c, 0x65, 0x6e, 0x73, 0x78, 0x49, 0x42, 0x5f, 0x54, 
    0xf7, 0xfc, 0xe1, 0xea, 0xdb, 0xd0, 0xcd, 0xc6, 0xaf, 0xa4, 0xb9, 0xb2, 0x83, 0x88, 0x95, 0x9e, 
    0x47, 0x4c, 0x51, 0x5a, 0x6b, 0x60, 0x7d, 0x76, 0x1f, 0x14, 0x09, 0x02, 0x33, 0x38, 0x25, 0x2e, 
    0x8c, 0x87, 0x9a, 0x91, 0xa0, 0xab, 0xb6, 0xbd, 0xd4, 0xdf, 0xc2, 0xc9, 0xf8, 0xf3, 0xee, 0xe5, 
    0x3c, 0x37, 0x2a, 0x21, 0x10, 0x1b, 0x06, 0x0d, 0x64, 0x6f, 0x72, 0x79, 0x48, 0x43, 0x5e, 0x55, 
    0x01, 0x0a, 0x17, 0x1c, 0x2d, 0x26, 0x3b, 0x30, 0x59, 0x52, 0x4f, 0x44, 0x75, 0x7e, 0x63, 0x68, 
    0xb1, 0xba, 0xa7, 0xac, 0x9d, 0x96, 0x8b, 0x80, 0xe9, 0xe2, 0xff, 0xf4, 0xc5, 0xce, 0xd3, 0xd8, 
    0x7a, 0x71, 0x6c, 0x67, 0x56, 0x5d, 0x40, 0x4b, 0x22, 0x29, 0x34, 0x3f, 0x0e, 0x05, 0x18, 0x13, 
    0xca, 0xc1, 0xdc, 0xd7, 0xe6, 0xed, 0xf0, 0xfb, 0x92, 0x99, 0x84, 0x8f, 0xbe, 0xb5, 0xa8, 0xa3
]; 
    
const XtimeD: [u8; 256] =
[
    0x00, 0x0d, 0x1a, 0x17, 0x34, 0x39, 0x2e, 0x23, 0x68, 0x65, 0x72, 0x7f, 0x5c, 0x51, 0x46, 0x4b, 
    0xd0, 0xdd, 0xca, 0xc7, 0xe4, 0xe9, 0xfe, 0xf3, 0xb8, 0xb5, 0xa2, 0xaf, 0x8c, 0x81, 0x96, 0x9b, 
    0xbb, 0xb6, 0xa1, 0xac, 0x8f, 0x82, 0x95, 0x98, 0xd3, 0xde, 0xc9, 0xc4, 0xe7, 0xea, 0xfd, 0xf0, 
    0x6b, 0x66, 0x71, 0x7c, 0x5f, 0x52, 0x45, 0x48, 0x03, 0x0e, 0x19, 0x14, 0x37, 0x3a, 0x2d, 0x20, 
    0x6d, 0x60, 0x77, 0x7a, 0x59, 0x54, 0x43, 0x4e, 0x05, 0x08, 0x1f, 0x12, 0x31, 0x3c, 0x2b, 0x26, 
    0xbd, 0xb0, 0xa7, 0xaa, 0x89, 0x84, 0x93, 0x9e, 0xd5, 0xd8, 0xcf, 0xc2, 0xe1, 0xec, 0xfb, 0xf6, 
    0xd6, 0xdb, 0xcc, 0xc1, 0xe2, 0xef, 0xf8, 0xf5, 0xbe, 0xb3, 0xa4, 0xa9, 0x8a, 0x87, 0x90, 0x9d, 
    0x06, 0x0b, 0x1c, 0x11, 0x32, 0x3f, 0x28, 0x25, 0x6e, 0x63, 0x74, 0x79, 0x5a, 0x57, 0x40, 0x4d, 
    0xda, 0xd7, 0xc0, 0xcd, 0xee, 0xe3, 0xf4, 0xf9, 0xb2, 0xbf, 0xa8, 0xa5, 0x86, 0x8b, 0x9c, 0x91, 
    0x0a, 0x07, 0x10, 0x1d, 0x3e, 0x33, 0x24, 0x29, 0x62, 0x6f, 0x78, 0x75, 0x56, 0x5b, 0x4c, 0x41, 
    0x61, 0x6c, 0x7b, 0x76, 0x55, 0x58, 0x4f, 0x42, 0x09, 0x04, 0x13, 0x1e, 0x3d, 0x30, 0x27, 0x2a, 
    0xb1, 0xbc, 0xab, 0xa6, 0x85, 0x88, 0x9f, 0x92, 0xd9, 0xd4, 0xc3, 0xce, 0xed, 0xe0, 0xf7, 0xfa, 
    0xb7, 0xba, 0xad, 0xa0, 0x83, 0x8e, 0x99, 0x94, 0xdf, 0xd2, 0xc5, 0xc8, 0xeb, 0xe6, 0xf1, 0xfc, 
    0x67, 0x6a, 0x7d, 0x70, 0x53, 0x5e, 0x49, 0x44, 0x0f, 0x02, 0x15, 0x18, 0x3b, 0x36, 0x21, 0x2c, 
    0x0c, 0x01, 0x16, 0x1b, 0x38, 0x35, 0x22, 0x2f, 0x64, 0x69, 0x7e, 0x73, 0x50, 0x5d, 0x4a, 0x47, 
    0xdc, 0xd1, 0xc6, 0xcb, 0xe8, 0xe5, 0xf2, 0xff, 0xb4, 0xb9, 0xae, 0xa3, 0x80, 0x8d, 0x9a, 0x97
]; 
    
const XtimeE: [u8; 256] =
[
    0x00, 0x0e, 0x1c, 0x12, 0x38, 0x36, 0x24, 0x2a, 0x70, 0x7e, 0x6c, 0x62, 0x48, 0x46, 0x54, 0x5a, 
    0xe0, 0xee, 0xfc, 0xf2, 0xd8, 0xd6, 0xc4, 0xca, 0x90, 0x9e, 0x8c, 0x82, 0xa8, 0xa6, 0xb4, 0xba, 
    0xdb, 0xd5, 0xc7, 0xc9, 0xe3, 0xed, 0xff, 0xf1, 0xab, 0xa5, 0xb7, 0xb9, 0x93, 0x9d, 0x8f, 0x81, 
    0x3b, 0x35, 0x27, 0x29, 0x03, 0x0d, 0x1f, 0x11, 0x4b, 0x45, 0x57, 0x59, 0x73, 0x7d, 0x6f, 0x61, 
    0xad, 0xa3, 0xb1, 0xbf, 0x95, 0x9b, 0x89, 0x87, 0xdd, 0xd3, 0xc1, 0xcf, 0xe5, 0xeb, 0xf9, 0xf7, 
    0x4d, 0x43, 0x51, 0x5f, 0x75, 0x7b, 0x69, 0x67, 0x3d, 0x33, 0x21, 0x2f, 0x05, 0x0b, 0x19, 0x17, 
    0x76, 0x78, 0x6a, 0x64, 0x4e, 0x40, 0x52, 0x5c, 0x06, 0x08, 0x1a, 0x14, 0x3e, 0x30, 0x22, 0x2c, 
    0x96, 0x98, 0x8a, 0x84, 0xae, 0xa0, 0xb2, 0xbc, 0xe6, 0xe8, 0xfa, 0xf4, 0xde, 0xd0, 0xc2, 0xcc, 
    0x41, 0x4f, 0x5d, 0x53, 0x79, 0x77, 0x65, 0x6b, 0x31, 0x3f, 0x2d, 0x23, 0x09, 0x07, 0x15, 0x1b, 
    0xa1, 0xaf, 0xbd, 0xb3, 0x99, 0x97, 0x85, 0x8b, 0xd1, 0xdf, 0xcd, 0xc3, 0xe9, 0xe7, 0xf5, 0xfb, 
    0x9a, 0x94, 0x86, 0x88, 0xa2, 0xac, 0xbe, 0xb0, 0xea, 0xe4, 0xf6, 0xf8, 0xd2, 0xdc, 0xce, 0xc0, 
    0x7a, 0x74, 0x66, 0x68, 0x42, 0x4c, 0x5e, 0x50, 0x0a, 0x04, 0x16, 0x18, 0x32, 0x3c, 0x2e, 0x20, 
    0xec, 0xe2, 0xf0, 0xfe, 0xd4, 0xda, 0xc8, 0xc6, 0x9c, 0x92, 0x80, 0x8e, 0xa4, 0xaa, 0xb8, 0xb6, 
    0x0c, 0x02, 0x10, 0x1e, 0x34, 0x3a, 0x28, 0x26, 0x7c, 0x72, 0x60, 0x6e, 0x44, 0x4a, 0x58, 0x56, 
    0x37, 0x39, 0x2b, 0x25, 0x0f, 0x01, 0x13, 0x1d, 0x47, 0x49, 0x5b, 0x55, 0x7f, 0x71, 0x63, 0x6d, 
    0xd7, 0xd9, 0xcb, 0xc5, 0xef, 0xe1, 0xf3, 0xfd, 0xa7, 0xa9, 0xbb, 0xb5, 0x9f, 0x91, 0x83, 0x8d
]; 

const Rcon: [u8; 11] = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
static mut XOR: [u8; 0x10000] = [0; 0x10000];
    
// exchanges columns in each of 4 rows
// row0 - unchanged, row1- shifted left 1, 
// row2 - shifted left 2 and row3 - shifted left 3
pub fn ShiftRows (state: &mut [u8; 16])
{
    let mut tmp: u8;

    // just substitute row 0
    state[0] = Sbox[state[0] as usize]; state[4] = Sbox[state[4] as usize];
    state[8] = Sbox[state[8] as usize]; state[12] = Sbox[state[12] as usize];

    // rotate row 1
    tmp = Sbox[state[1] as usize]; state[1] = Sbox[state[5] as usize];
    state[5] = Sbox[state[9] as usize]; state[9] = Sbox[state[13] as usize]; state[13] = tmp;

    // rotate row 2
    tmp = Sbox[state[2] as usize]; state[2] = Sbox[state[10] as usize]; state[10] = tmp;
    tmp = Sbox[state[6] as usize]; state[6] = Sbox[state[14] as usize]; state[14] = tmp;

    // rotate row 3
    tmp = Sbox[state[15] as usize]; state[15] = Sbox[state[11] as usize];
    state[11] = Sbox[state[7] as usize]; state[7] = Sbox[state[3] as usize]; state[3] = tmp;
}

// restores columns in each of 4 rows
// row0 - unchanged, row1- shifted right 1, 
// row2 - shifted right 2 and row3 - shifted right 3
pub fn InvShiftRows (state: &mut [u8; 16])
{
    let mut tmp: u8;

    // restore row 0
    state[0] = InvSbox[state[0] as usize]; state[4] = InvSbox[state[4] as usize];
    state[8] = InvSbox[state[8] as usize]; state[12] = InvSbox[state[12] as usize];

    // restore row 1
    tmp = InvSbox[state[13] as usize]; state[13] = InvSbox[state[9] as usize];
    state[9] = InvSbox[state[5] as usize]; state[5] = InvSbox[state[1] as usize]; state[1] = tmp;

    // restore row 2
    tmp = InvSbox[state[2] as usize]; state[2] = InvSbox[state[10] as usize]; state[10] = tmp;
    tmp = InvSbox[state[6] as usize]; state[6] = InvSbox[state[14] as usize]; state[14] = tmp;

    // restore row 3
    tmp = InvSbox[state[3] as usize]; state[3] = InvSbox[state[7] as usize];
    state[7] = InvSbox[state[11] as usize]; state[11] = InvSbox[state[15] as usize]; state[15] = tmp;
}

// recombine and mix each row in a column
pub fn MixSubColumns (state: &mut [u8; 16])
{
    let mut tmp: [u8; 16] = [0; 16];

    // mixing column 0
    tmp[0] = xor(Xtime2Sbox[state[0] as usize], Xtime3Sbox[state[5] as usize], Sbox[state[10] as usize], Sbox[state[15] as usize]);
    tmp[1] = xor(Sbox[state[0] as usize], Xtime2Sbox[state[5] as usize], Xtime3Sbox[state[10] as usize], Sbox[state[15] as usize]);
    tmp[2] = xor(Sbox[state[0] as usize], Sbox[state[5] as usize], Xtime2Sbox[state[10] as usize], Xtime3Sbox[state[15] as usize]);
    tmp[3] = xor(Xtime3Sbox[state[0] as usize], Sbox[state[5] as usize], Sbox[state[10] as usize], Xtime2Sbox[state[15] as usize]);

    // mixing column 1
    tmp[4] = xor(Xtime2Sbox[state[4] as usize], Xtime3Sbox[state[9] as usize], Sbox[state[14] as usize], Sbox[state[3] as usize]);
    tmp[5] = xor(Sbox[state[4] as usize], Xtime2Sbox[state[9] as usize], Xtime3Sbox[state[14] as usize], Sbox[state[3] as usize]);
    tmp[6] = xor(Sbox[state[4] as usize], Sbox[state[9] as usize], Xtime2Sbox[state[14] as usize], Xtime3Sbox[state[3] as usize]);
    tmp[7] = xor(Xtime3Sbox[state[4] as usize], Sbox[state[9] as usize], Sbox[state[14] as usize], Xtime2Sbox[state[3] as usize]);

    // mixing column 2
    tmp[8] = xor(Xtime2Sbox[state[8] as usize], Xtime3Sbox[state[13] as usize], Sbox[state[2] as usize], Sbox[state[7] as usize]);
    tmp[9] = xor(Sbox[state[8] as usize], Xtime2Sbox[state[13] as usize], Xtime3Sbox[state[2] as usize], Sbox[state[7] as usize]);
    tmp[10] = xor(Sbox[state[8] as usize], Sbox[state[13] as usize], Xtime2Sbox[state[2] as usize], Xtime3Sbox[state[7] as usize]);
    tmp[11] = xor(Xtime3Sbox[state[8] as usize], Sbox[state[13] as usize], Sbox[state[2] as usize], Xtime2Sbox[state[7] as usize]);

    // mixing column 3
    tmp[12] = xor(Xtime2Sbox[state[12] as usize], Xtime3Sbox[state[1] as usize], Sbox[state[6] as usize], Sbox[state[11] as usize]);
    tmp[13] = xor(Sbox[state[12] as usize], Xtime2Sbox[state[1] as usize], Xtime3Sbox[state[6] as usize], Sbox[state[11] as usize]);
    tmp[14] = xor(Sbox[state[12] as usize], Sbox[state[1] as usize], Xtime2Sbox[state[6] as usize], Xtime3Sbox[state[11] as usize]);
    tmp[15] = xor(Xtime3Sbox[state[12] as usize], Sbox[state[1] as usize], Sbox[state[6] as usize], Xtime2Sbox[state[11] as usize]);

    state.iter_mut().zip(tmp.iter()).for_each(|(s, t)| *s = *t);
}

// restore and un-mix each row in a column
pub fn InvMixSubColumns (state: &mut [u8; 16])
{
    let mut tmp: [u8; 16] = [0; 16];

    // restore column 0
    tmp[0] = xor(XtimeE[state[0] as usize], XtimeB[state[1] as usize], XtimeD[state[2] as usize], Xtime9[state[3] as usize]);
    tmp[5] = xor(Xtime9[state[0] as usize], XtimeE[state[1] as usize], XtimeB[state[2] as usize], XtimeD[state[3] as usize]);
    tmp[10] = xor(XtimeD[state[0] as usize], Xtime9[state[1] as usize], XtimeE[state[2] as usize], XtimeB[state[3] as usize]);
    tmp[15] = xor(XtimeB[state[0] as usize], XtimeD[state[1] as usize], Xtime9[state[2] as usize], XtimeE[state[3] as usize]);

    // restore column 1
    tmp[4] = xor(XtimeE[state[4] as usize], XtimeB[state[5] as usize], XtimeD[state[6] as usize], Xtime9[state[7] as usize]);
    tmp[9] = xor(Xtime9[state[4] as usize], XtimeE[state[5] as usize], XtimeB[state[6] as usize], XtimeD[state[7] as usize]);
    tmp[14] = xor(XtimeD[state[4] as usize], Xtime9[state[5] as usize], XtimeE[state[6] as usize], XtimeB[state[7] as usize]);
    tmp[3] = xor(XtimeB[state[4] as usize], XtimeD[state[5] as usize], Xtime9[state[6] as usize], XtimeE[state[7] as usize]);

    // restore column 2
    tmp[8] = xor(XtimeE[state[8] as usize], XtimeB[state[9] as usize], XtimeD[state[10] as usize], Xtime9[state[11] as usize]);
    tmp[13] = xor(Xtime9[state[8] as usize], XtimeE[state[9] as usize], XtimeB[state[10] as usize], XtimeD[state[11] as usize]);
    tmp[2]  = xor(XtimeD[state[8] as usize], Xtime9[state[9] as usize], XtimeE[state[10] as usize], XtimeB[state[11] as usize]);
    tmp[7]  = xor(XtimeB[state[8] as usize], XtimeD[state[9] as usize], Xtime9[state[10] as usize], XtimeE[state[11] as usize]);

    // restore column 3
    tmp[12] = xor(XtimeE[state[12] as usize], XtimeB[state[13] as usize], XtimeD[state[14] as usize], Xtime9[state[15] as usize]);
    tmp[1] = xor(Xtime9[state[12] as usize], XtimeE[state[13] as usize], XtimeB[state[14] as usize], XtimeD[state[15] as usize]);
    tmp[6] = xor(XtimeD[state[12] as usize], Xtime9[state[13] as usize], XtimeE[state[14] as usize], XtimeB[state[15] as usize]);
    tmp[11] = xor(XtimeB[state[12] as usize], XtimeD[state[13] as usize], Xtime9[state[14] as usize], XtimeE[state[15] as usize]);

    state.iter_mut().zip(tmp.iter()).for_each(|(s, t)| *s = InvSbox[*t as usize]);
}

// encrypt/decrypt columns with the key

pub fn AddRoundKey (state: &mut [u8; 16], key: [u8; 16])
{
    state.iter_mut().zip(key.iter()).for_each(|(s, k)| *s = Xor(*s, *k));
}

// compute aes key schedule
pub fn ExpandKey (key: [u8; 16]) -> [u8; 176]
{
    let mut expkey: [u8; 176] = [0; 176];
    let mut tmp0: u8;
    let mut tmp1: u8;
    let mut tmp2: u8;
    let mut tmp3: u8;
    let mut tmp4: u8;

    key.iter().zip(expkey.iter_mut()).for_each(|(k, ek)| *ek = *k);

    for idx in 4..44
    {
        tmp0 = expkey[4*idx - 4];
        tmp1 = expkey[4*idx - 3];
        tmp2 = expkey[4*idx - 2];
        tmp3 = expkey[4*idx - 1];
        if idx % 4 == 0
        {
            tmp4 = tmp3;
            tmp3 = Sbox[tmp0 as usize];
            tmp0 = Xor(Sbox[tmp1 as usize], Rcon[idx/4]);
            tmp1 = Sbox[tmp2 as usize];
            tmp2 = Sbox[tmp4 as usize];
        }

        expkey[4*idx+0] = Xor(expkey[4*idx - 16 + 0], tmp0);
        expkey[4*idx+1] = Xor(expkey[4*idx - 16 + 1], tmp1);
        expkey[4*idx+2] = Xor(expkey[4*idx - 16 + 2], tmp2);
        expkey[4*idx+3] = Xor(expkey[4*idx - 16 + 3], tmp3);
    }
    expkey
}

pub fn xor (x: u8, y: u8, z: u8, t: u8) -> u8
{
    unsafe {Xor(XOR[y as usize | ((x as usize) << 8)], XOR[z as usize | ((t as usize) << 8)])}
}

pub fn Xor (x: u8, y: u8) -> u8
{
    unsafe {XOR[y as usize | ((x as usize) << 8)]}
}

pub struct AesCipher
{
    pub expkey: [u8; 176]
}

impl AesCipher
{
    pub fn create(key: [u8; 16]) -> AesCipher
    {
        // compute the key schedule
        AesCipher
        {
            expkey: ExpandKey (key)
        }
    }

    // encrypt one 128 bit block
    pub fn EncryptBlock (&self, pt: [u8; 16]) -> [u8; 16]
    {
        let mut state = pt.clone();
        AddRoundKey (&mut state, self.expkey[0..16].try_into().unwrap());

        for round in 1..11
        {
            if round < 10 {MixSubColumns (&mut state)}
            else {ShiftRows (&mut state)}
            AddRoundKey (&mut state, self.expkey[round*16..round*16+16].try_into().unwrap());
        }

        state
    }

    pub fn DecryptBlock (&self, ct: [u8; 16]) -> [u8; 16]
    {
        let mut state = ct.clone();
        AddRoundKey (&mut state, self.expkey[160..176].try_into().unwrap());
        InvShiftRows(&mut state);

        for round in (0..10).rev()
        {
            AddRoundKey (&mut state, self.expkey[round*16..round*16+16].try_into().unwrap());
            if round > 0 {InvMixSubColumns (&mut state)}
        } 

        state
    }

    pub fn Encrypt (&self, pt: &Vec<u8>) -> Vec<u8>
    {
        let mut state = pt.clone();
        let mut len =  pt.len();
        if len % 16 > 0
        {
            len = len + 16 - len % 16;
            state.resize(len, 0);
        }
        let mut ct = Vec::<u8>::with_capacity(len);

        for i in (0..len).step_by(16)
        {
            let out = self.EncryptBlock(state[i..i+16].try_into().unwrap());
            ct.append(&mut out.to_vec());
        }
        ct
    }

    pub fn Decrypt (&self, ct: &Vec<u8>) -> Vec<u8>
    {
        let mut pt = Vec::<u8>::with_capacity(ct.len());
        for i in (0..ct.len()).step_by(16)
        {
            let out = self.DecryptBlock(ct[i..i+16].try_into().unwrap());
            pt.append(&mut out.to_vec());
        }
        pt
    }
}

#[test]
fn aes()
{
    let rng = &mut thread_rng();
    for x in 0..256
    {
        for y in 0..256
        {
            unsafe {XOR[y | (x << 8)] = (x as u8) ^ (y as u8)}
        }
    }

    let key: u128 = rng.gen();
    let cipher = AesCipher::create(key.to_ne_bytes());

    let mut pt1 = (0..1000).map(|_| {let x: u8 = rng.gen(); x}).collect::<Vec<u8>>();

    let ct1 = cipher.Encrypt(&pt1);
    let pt2 = cipher.Decrypt(&ct1);

    let ct2 = cipher.Encrypt(&pt2);
    pt1 = cipher.Decrypt(&ct2);

    assert_eq!(pt1, pt2);
    assert_eq!(ct1, ct2);
}
