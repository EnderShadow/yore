// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/
// See binary codegen crate
use crate::internal::{DecoderComplete, Encoder};
use crate::{CodePage, DecodeError, EncodeError};
use std::borrow::Cow;

#[derive(Copy, Clone)]
pub struct CP437;

impl CP437 {
    /// Decode CP437 byte-encoding into UTF-8 string
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP437;
    ///
    /// assert_eq!(CP437.decode(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Cow<str> {
        Self::decode_helper(bytes)
    }

    /// Encode UTF-8 string into CP437 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP437;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP437.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP437.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CP437 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP437;
    ///
    /// assert_eq!(CP437.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}
impl CodePage for CP437 {
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        Ok((*self).decode(bytes))
    }

    #[inline(always)]
    fn decode_lossy<'a>(&self, bytes: &'a [u8]) -> Cow<'a, str> {
        (*self).decode(bytes)
    }

    #[inline(always)]
    fn decode_lossy_fallback<'a>(&self, bytes: &'a [u8], _fallback: char) -> Cow<'a, str> {
        (*self).decode(bytes)
    }
}

impl DecoderComplete for CP437 {
    const DECODE_TABLE: [([u8; 3], u8); 256] = [
        ([0x0, 0x0, 0x0], 1),
        ([0x1, 0x0, 0x0], 1),
        ([0x2, 0x0, 0x0], 1),
        ([0x3, 0x0, 0x0], 1),
        ([0x4, 0x0, 0x0], 1),
        ([0x5, 0x0, 0x0], 1),
        ([0x6, 0x0, 0x0], 1),
        ([0x7, 0x0, 0x0], 1),
        ([0x8, 0x0, 0x0], 1),
        ([0x9, 0x0, 0x0], 1),
        ([0xA, 0x0, 0x0], 1),
        ([0xB, 0x0, 0x0], 1),
        ([0xC, 0x0, 0x0], 1),
        ([0xD, 0x0, 0x0], 1),
        ([0xE, 0x0, 0x0], 1),
        ([0xF, 0x0, 0x0], 1),
        ([0x10, 0x0, 0x0], 1),
        ([0x11, 0x0, 0x0], 1),
        ([0x12, 0x0, 0x0], 1),
        ([0x13, 0x0, 0x0], 1),
        ([0x14, 0x0, 0x0], 1),
        ([0x15, 0x0, 0x0], 1),
        ([0x16, 0x0, 0x0], 1),
        ([0x17, 0x0, 0x0], 1),
        ([0x18, 0x0, 0x0], 1),
        ([0x19, 0x0, 0x0], 1),
        ([0x1A, 0x0, 0x0], 1),
        ([0x1B, 0x0, 0x0], 1),
        ([0x1C, 0x0, 0x0], 1),
        ([0x1D, 0x0, 0x0], 1),
        ([0x1E, 0x0, 0x0], 1),
        ([0x1F, 0x0, 0x0], 1),
        ([0x20, 0x0, 0x0], 1),
        ([0x21, 0x0, 0x0], 1),
        ([0x22, 0x0, 0x0], 1),
        ([0x23, 0x0, 0x0], 1),
        ([0x24, 0x0, 0x0], 1),
        ([0x25, 0x0, 0x0], 1),
        ([0x26, 0x0, 0x0], 1),
        ([0x27, 0x0, 0x0], 1),
        ([0x28, 0x0, 0x0], 1),
        ([0x29, 0x0, 0x0], 1),
        ([0x2A, 0x0, 0x0], 1),
        ([0x2B, 0x0, 0x0], 1),
        ([0x2C, 0x0, 0x0], 1),
        ([0x2D, 0x0, 0x0], 1),
        ([0x2E, 0x0, 0x0], 1),
        ([0x2F, 0x0, 0x0], 1),
        ([0x30, 0x0, 0x0], 1),
        ([0x31, 0x0, 0x0], 1),
        ([0x32, 0x0, 0x0], 1),
        ([0x33, 0x0, 0x0], 1),
        ([0x34, 0x0, 0x0], 1),
        ([0x35, 0x0, 0x0], 1),
        ([0x36, 0x0, 0x0], 1),
        ([0x37, 0x0, 0x0], 1),
        ([0x38, 0x0, 0x0], 1),
        ([0x39, 0x0, 0x0], 1),
        ([0x3A, 0x0, 0x0], 1),
        ([0x3B, 0x0, 0x0], 1),
        ([0x3C, 0x0, 0x0], 1),
        ([0x3D, 0x0, 0x0], 1),
        ([0x3E, 0x0, 0x0], 1),
        ([0x3F, 0x0, 0x0], 1),
        ([0x40, 0x0, 0x0], 1),
        ([0x41, 0x0, 0x0], 1),
        ([0x42, 0x0, 0x0], 1),
        ([0x43, 0x0, 0x0], 1),
        ([0x44, 0x0, 0x0], 1),
        ([0x45, 0x0, 0x0], 1),
        ([0x46, 0x0, 0x0], 1),
        ([0x47, 0x0, 0x0], 1),
        ([0x48, 0x0, 0x0], 1),
        ([0x49, 0x0, 0x0], 1),
        ([0x4A, 0x0, 0x0], 1),
        ([0x4B, 0x0, 0x0], 1),
        ([0x4C, 0x0, 0x0], 1),
        ([0x4D, 0x0, 0x0], 1),
        ([0x4E, 0x0, 0x0], 1),
        ([0x4F, 0x0, 0x0], 1),
        ([0x50, 0x0, 0x0], 1),
        ([0x51, 0x0, 0x0], 1),
        ([0x52, 0x0, 0x0], 1),
        ([0x53, 0x0, 0x0], 1),
        ([0x54, 0x0, 0x0], 1),
        ([0x55, 0x0, 0x0], 1),
        ([0x56, 0x0, 0x0], 1),
        ([0x57, 0x0, 0x0], 1),
        ([0x58, 0x0, 0x0], 1),
        ([0x59, 0x0, 0x0], 1),
        ([0x5A, 0x0, 0x0], 1),
        ([0x5B, 0x0, 0x0], 1),
        ([0x5C, 0x0, 0x0], 1),
        ([0x5D, 0x0, 0x0], 1),
        ([0x5E, 0x0, 0x0], 1),
        ([0x5F, 0x0, 0x0], 1),
        ([0x60, 0x0, 0x0], 1),
        ([0x61, 0x0, 0x0], 1),
        ([0x62, 0x0, 0x0], 1),
        ([0x63, 0x0, 0x0], 1),
        ([0x64, 0x0, 0x0], 1),
        ([0x65, 0x0, 0x0], 1),
        ([0x66, 0x0, 0x0], 1),
        ([0x67, 0x0, 0x0], 1),
        ([0x68, 0x0, 0x0], 1),
        ([0x69, 0x0, 0x0], 1),
        ([0x6A, 0x0, 0x0], 1),
        ([0x6B, 0x0, 0x0], 1),
        ([0x6C, 0x0, 0x0], 1),
        ([0x6D, 0x0, 0x0], 1),
        ([0x6E, 0x0, 0x0], 1),
        ([0x6F, 0x0, 0x0], 1),
        ([0x70, 0x0, 0x0], 1),
        ([0x71, 0x0, 0x0], 1),
        ([0x72, 0x0, 0x0], 1),
        ([0x73, 0x0, 0x0], 1),
        ([0x74, 0x0, 0x0], 1),
        ([0x75, 0x0, 0x0], 1),
        ([0x76, 0x0, 0x0], 1),
        ([0x77, 0x0, 0x0], 1),
        ([0x78, 0x0, 0x0], 1),
        ([0x79, 0x0, 0x0], 1),
        ([0x7A, 0x0, 0x0], 1),
        ([0x7B, 0x0, 0x0], 1),
        ([0x7C, 0x0, 0x0], 1),
        ([0x7D, 0x0, 0x0], 1),
        ([0x7E, 0x0, 0x0], 1),
        ([0x7F, 0x0, 0x0], 1),
        ([0xC3, 0x87, 0x0], 2),
        ([0xC3, 0xBC, 0x0], 2),
        ([0xC3, 0xA9, 0x0], 2),
        ([0xC3, 0xA2, 0x0], 2),
        ([0xC3, 0xA4, 0x0], 2),
        ([0xC3, 0xA0, 0x0], 2),
        ([0xC3, 0xA5, 0x0], 2),
        ([0xC3, 0xA7, 0x0], 2),
        ([0xC3, 0xAA, 0x0], 2),
        ([0xC3, 0xAB, 0x0], 2),
        ([0xC3, 0xA8, 0x0], 2),
        ([0xC3, 0xAF, 0x0], 2),
        ([0xC3, 0xAE, 0x0], 2),
        ([0xC3, 0xAC, 0x0], 2),
        ([0xC3, 0x84, 0x0], 2),
        ([0xC3, 0x85, 0x0], 2),
        ([0xC3, 0x89, 0x0], 2),
        ([0xC3, 0xA6, 0x0], 2),
        ([0xC3, 0x86, 0x0], 2),
        ([0xC3, 0xB4, 0x0], 2),
        ([0xC3, 0xB6, 0x0], 2),
        ([0xC3, 0xB2, 0x0], 2),
        ([0xC3, 0xBB, 0x0], 2),
        ([0xC3, 0xB9, 0x0], 2),
        ([0xC3, 0xBF, 0x0], 2),
        ([0xC3, 0x96, 0x0], 2),
        ([0xC3, 0x9C, 0x0], 2),
        ([0xC2, 0xA2, 0x0], 2),
        ([0xC2, 0xA3, 0x0], 2),
        ([0xC2, 0xA5, 0x0], 2),
        ([0xE2, 0x82, 0xA7], 3),
        ([0xC6, 0x92, 0x0], 2),
        ([0xC3, 0xA1, 0x0], 2),
        ([0xC3, 0xAD, 0x0], 2),
        ([0xC3, 0xB3, 0x0], 2),
        ([0xC3, 0xBA, 0x0], 2),
        ([0xC3, 0xB1, 0x0], 2),
        ([0xC3, 0x91, 0x0], 2),
        ([0xC2, 0xAA, 0x0], 2),
        ([0xC2, 0xBA, 0x0], 2),
        ([0xC2, 0xBF, 0x0], 2),
        ([0xE2, 0x8C, 0x90], 3),
        ([0xC2, 0xAC, 0x0], 2),
        ([0xC2, 0xBD, 0x0], 2),
        ([0xC2, 0xBC, 0x0], 2),
        ([0xC2, 0xA1, 0x0], 2),
        ([0xC2, 0xAB, 0x0], 2),
        ([0xC2, 0xBB, 0x0], 2),
        ([0xE2, 0x96, 0x91], 3),
        ([0xE2, 0x96, 0x92], 3),
        ([0xE2, 0x96, 0x93], 3),
        ([0xE2, 0x94, 0x82], 3),
        ([0xE2, 0x94, 0xA4], 3),
        ([0xE2, 0x95, 0xA1], 3),
        ([0xE2, 0x95, 0xA2], 3),
        ([0xE2, 0x95, 0x96], 3),
        ([0xE2, 0x95, 0x95], 3),
        ([0xE2, 0x95, 0xA3], 3),
        ([0xE2, 0x95, 0x91], 3),
        ([0xE2, 0x95, 0x97], 3),
        ([0xE2, 0x95, 0x9D], 3),
        ([0xE2, 0x95, 0x9C], 3),
        ([0xE2, 0x95, 0x9B], 3),
        ([0xE2, 0x94, 0x90], 3),
        ([0xE2, 0x94, 0x94], 3),
        ([0xE2, 0x94, 0xB4], 3),
        ([0xE2, 0x94, 0xAC], 3),
        ([0xE2, 0x94, 0x9C], 3),
        ([0xE2, 0x94, 0x80], 3),
        ([0xE2, 0x94, 0xBC], 3),
        ([0xE2, 0x95, 0x9E], 3),
        ([0xE2, 0x95, 0x9F], 3),
        ([0xE2, 0x95, 0x9A], 3),
        ([0xE2, 0x95, 0x94], 3),
        ([0xE2, 0x95, 0xA9], 3),
        ([0xE2, 0x95, 0xA6], 3),
        ([0xE2, 0x95, 0xA0], 3),
        ([0xE2, 0x95, 0x90], 3),
        ([0xE2, 0x95, 0xAC], 3),
        ([0xE2, 0x95, 0xA7], 3),
        ([0xE2, 0x95, 0xA8], 3),
        ([0xE2, 0x95, 0xA4], 3),
        ([0xE2, 0x95, 0xA5], 3),
        ([0xE2, 0x95, 0x99], 3),
        ([0xE2, 0x95, 0x98], 3),
        ([0xE2, 0x95, 0x92], 3),
        ([0xE2, 0x95, 0x93], 3),
        ([0xE2, 0x95, 0xAB], 3),
        ([0xE2, 0x95, 0xAA], 3),
        ([0xE2, 0x94, 0x98], 3),
        ([0xE2, 0x94, 0x8C], 3),
        ([0xE2, 0x96, 0x88], 3),
        ([0xE2, 0x96, 0x84], 3),
        ([0xE2, 0x96, 0x8C], 3),
        ([0xE2, 0x96, 0x90], 3),
        ([0xE2, 0x96, 0x80], 3),
        ([0xCE, 0xB1, 0x0], 2),
        ([0xC3, 0x9F, 0x0], 2),
        ([0xCE, 0x93, 0x0], 2),
        ([0xCF, 0x80, 0x0], 2),
        ([0xCE, 0xA3, 0x0], 2),
        ([0xCF, 0x83, 0x0], 2),
        ([0xC2, 0xB5, 0x0], 2),
        ([0xCF, 0x84, 0x0], 2),
        ([0xCE, 0xA6, 0x0], 2),
        ([0xCE, 0x98, 0x0], 2),
        ([0xCE, 0xA9, 0x0], 2),
        ([0xCE, 0xB4, 0x0], 2),
        ([0xE2, 0x88, 0x9E], 3),
        ([0xCF, 0x86, 0x0], 2),
        ([0xCE, 0xB5, 0x0], 2),
        ([0xE2, 0x88, 0xA9], 3),
        ([0xE2, 0x89, 0xA1], 3),
        ([0xC2, 0xB1, 0x0], 2),
        ([0xE2, 0x89, 0xA5], 3),
        ([0xE2, 0x89, 0xA4], 3),
        ([0xE2, 0x8C, 0xA0], 3),
        ([0xE2, 0x8C, 0xA1], 3),
        ([0xC3, 0xB7, 0x0], 2),
        ([0xE2, 0x89, 0x88], 3),
        ([0xC2, 0xB0, 0x0], 2),
        ([0xE2, 0x88, 0x99], 3),
        ([0xC2, 0xB7, 0x0], 2),
        ([0xE2, 0x88, 0x9A], 3),
        ([0xE2, 0x81, 0xBF], 3),
        ([0xC2, 0xB2, 0x0], 2),
        ([0xE2, 0x96, 0xA0], 3),
        ([0xC2, 0xA0, 0x0], 2),
    ];
}
impl Encoder for CP437 {
    #[doc(hidden)]
    #[inline]
    fn encode_grapheme(&self, bytes: &mut &[u8]) -> Option<u8> {
        let (&a, rest) = bytes.split_first().unwrap();
        Some(match (a, &bytes) {
            (0x00..=0x7F, _) => {
                *bytes = rest;
                a
            }
            (0xC2, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0xA2 => 0x9B,
                    0xA3 => 0x9C,
                    0xA5 => 0x9D,
                    0xAA => 0xA6,
                    0xBA => 0xA7,
                    0xBF => 0xA8,
                    0xAC => 0xAA,
                    0xBD => 0xAB,
                    0xBC => 0xAC,
                    0xA1 => 0xAD,
                    0xAB => 0xAE,
                    0xBB => 0xAF,
                    0xB5 => 0xE6,
                    0xB1 => 0xF1,
                    0xB0 => 0xF8,
                    0xB7 => 0xFA,
                    0xB2 => 0xFD,
                    0xA0 => 0xFF,
                    _ => return None,
                }
            }
            (0xC3, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x87 => 0x80,
                    0xBC => 0x81,
                    0xA9 => 0x82,
                    0xA2 => 0x83,
                    0xA4 => 0x84,
                    0xA0 => 0x85,
                    0xA5 => 0x86,
                    0xA7 => 0x87,
                    0xAA => 0x88,
                    0xAB => 0x89,
                    0xA8 => 0x8A,
                    0xAF => 0x8B,
                    0xAE => 0x8C,
                    0xAC => 0x8D,
                    0x84 => 0x8E,
                    0x85 => 0x8F,
                    0x89 => 0x90,
                    0xA6 => 0x91,
                    0x86 => 0x92,
                    0xB4 => 0x93,
                    0xB6 => 0x94,
                    0xB2 => 0x95,
                    0xBB => 0x96,
                    0xB9 => 0x97,
                    0xBF => 0x98,
                    0x96 => 0x99,
                    0x9C => 0x9A,
                    0xA1 => 0xA0,
                    0xAD => 0xA1,
                    0xB3 => 0xA2,
                    0xBA => 0xA3,
                    0xB1 => 0xA4,
                    0x91 => 0xA5,
                    0x9F => 0xE1,
                    0xB7 => 0xF6,
                    _ => return None,
                }
            }
            (0xC6, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x92 => 0x9F,
                    _ => return None,
                }
            }
            (0xCE, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0xB1 => 0xE0,
                    0x93 => 0xE2,
                    0xA3 => 0xE4,
                    0xA6 => 0xE8,
                    0x98 => 0xE9,
                    0xA9 => 0xEA,
                    0xB4 => 0xEB,
                    0xB5 => 0xEE,
                    _ => return None,
                }
            }
            (0xCF, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x80 => 0xE3,
                    0x83 => 0xE5,
                    0x84 => 0xE7,
                    0x86 => 0xED,
                    _ => return None,
                }
            }
            (0xE2, [_, b, c, ..]) => {
                *bytes = &bytes[3..];
                match b {
                    0x81 => match c {
                        0xBF => 0xFC,
                        _ => return None,
                    },
                    0x82 => match c {
                        0xA7 => 0x9E,
                        _ => return None,
                    },
                    0x88 => match c {
                        0x9E => 0xEC,
                        0xA9 => 0xEF,
                        0x99 => 0xF9,
                        0x9A => 0xFB,
                        _ => return None,
                    },
                    0x89 => match c {
                        0xA1 => 0xF0,
                        0xA5 => 0xF2,
                        0xA4 => 0xF3,
                        0x88 => 0xF7,
                        _ => return None,
                    },
                    0x8C => match c {
                        0x90 => 0xA9,
                        0xA0 => 0xF4,
                        0xA1 => 0xF5,
                        _ => return None,
                    },
                    0x94 => match c {
                        0x82 => 0xB3,
                        0xA4 => 0xB4,
                        0x90 => 0xBF,
                        0x94 => 0xC0,
                        0xB4 => 0xC1,
                        0xAC => 0xC2,
                        0x9C => 0xC3,
                        0x80 => 0xC4,
                        0xBC => 0xC5,
                        0x98 => 0xD9,
                        0x8C => 0xDA,
                        _ => return None,
                    },
                    0x95 => match c {
                        0xA1 => 0xB5,
                        0xA2 => 0xB6,
                        0x96 => 0xB7,
                        0x95 => 0xB8,
                        0xA3 => 0xB9,
                        0x91 => 0xBA,
                        0x97 => 0xBB,
                        0x9D => 0xBC,
                        0x9C => 0xBD,
                        0x9B => 0xBE,
                        0x9E => 0xC6,
                        0x9F => 0xC7,
                        0x9A => 0xC8,
                        0x94 => 0xC9,
                        0xA9 => 0xCA,
                        0xA6 => 0xCB,
                        0xA0 => 0xCC,
                        0x90 => 0xCD,
                        0xAC => 0xCE,
                        0xA7 => 0xCF,
                        0xA8 => 0xD0,
                        0xA4 => 0xD1,
                        0xA5 => 0xD2,
                        0x99 => 0xD3,
                        0x98 => 0xD4,
                        0x92 => 0xD5,
                        0x93 => 0xD6,
                        0xAB => 0xD7,
                        0xAA => 0xD8,
                        _ => return None,
                    },
                    0x96 => match c {
                        0x91 => 0xB0,
                        0x92 => 0xB1,
                        0x93 => 0xB2,
                        0x88 => 0xDB,
                        0x84 => 0xDC,
                        0x8C => 0xDD,
                        0x90 => 0xDE,
                        0x80 => 0xDF,
                        0xA0 => 0xFE,
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            (0xC2..=0xDF, _) => {
                *bytes = &bytes[2..];
                return None;
            }
            (0xE0..=0xEF, _) => {
                *bytes = &bytes[3..];
                return None;
            }
            (0xF0..=0xF4, _) => {
                *bytes = &bytes[4..];
                return None;
            }
            _ => panic!(),
        })
    }
}
