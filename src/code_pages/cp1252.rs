// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/
// See binary codegen crate
use crate::internal::decoder_complete;
use crate::internal::decoder_complete::decode_helper;
use crate::internal::Encoder;
use crate::{CodePage, DecodeError, EncodeError};
use std::borrow::Cow;

#[derive(Copy, Clone)]
pub struct CP1252;

impl CP1252 {
    /// Decode CP1252 byte-encoding into UTF-8 string
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1252;
    ///
    /// assert_eq!(CP1252.decode(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Cow<str> {
        decode_helper(&DECODE_TABLE, bytes)
    }

    /// Encode UTF-8 string into CP1252 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1252;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP1252.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP1252.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CP1252 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1252;
    ///
    /// assert_eq!(CP1252.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}
impl CodePage for CP1252 {
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

const DECODE_TABLE: decoder_complete::Table = [
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
    ([0xE2, 0x82, 0xAC], 3),
    ([0xC2, 0x81, 0x0], 2),
    ([0xE2, 0x80, 0x9A], 3),
    ([0xC6, 0x92, 0x0], 2),
    ([0xE2, 0x80, 0x9E], 3),
    ([0xE2, 0x80, 0xA6], 3),
    ([0xE2, 0x80, 0xA0], 3),
    ([0xE2, 0x80, 0xA1], 3),
    ([0xCB, 0x86, 0x0], 2),
    ([0xE2, 0x80, 0xB0], 3),
    ([0xC5, 0xA0, 0x0], 2),
    ([0xE2, 0x80, 0xB9], 3),
    ([0xC5, 0x92, 0x0], 2),
    ([0xC2, 0x8D, 0x0], 2),
    ([0xC5, 0xBD, 0x0], 2),
    ([0xC2, 0x8F, 0x0], 2),
    ([0xC2, 0x90, 0x0], 2),
    ([0xE2, 0x80, 0x98], 3),
    ([0xE2, 0x80, 0x99], 3),
    ([0xE2, 0x80, 0x9C], 3),
    ([0xE2, 0x80, 0x9D], 3),
    ([0xE2, 0x80, 0xA2], 3),
    ([0xE2, 0x80, 0x93], 3),
    ([0xE2, 0x80, 0x94], 3),
    ([0xCB, 0x9C, 0x0], 2),
    ([0xE2, 0x84, 0xA2], 3),
    ([0xC5, 0xA1, 0x0], 2),
    ([0xE2, 0x80, 0xBA], 3),
    ([0xC5, 0x93, 0x0], 2),
    ([0xC2, 0x9D, 0x0], 2),
    ([0xC5, 0xBE, 0x0], 2),
    ([0xC5, 0xB8, 0x0], 2),
    ([0xC2, 0xA0, 0x0], 2),
    ([0xC2, 0xA1, 0x0], 2),
    ([0xC2, 0xA2, 0x0], 2),
    ([0xC2, 0xA3, 0x0], 2),
    ([0xC2, 0xA4, 0x0], 2),
    ([0xC2, 0xA5, 0x0], 2),
    ([0xC2, 0xA6, 0x0], 2),
    ([0xC2, 0xA7, 0x0], 2),
    ([0xC2, 0xA8, 0x0], 2),
    ([0xC2, 0xA9, 0x0], 2),
    ([0xC2, 0xAA, 0x0], 2),
    ([0xC2, 0xAB, 0x0], 2),
    ([0xC2, 0xAC, 0x0], 2),
    ([0xC2, 0xAD, 0x0], 2),
    ([0xC2, 0xAE, 0x0], 2),
    ([0xC2, 0xAF, 0x0], 2),
    ([0xC2, 0xB0, 0x0], 2),
    ([0xC2, 0xB1, 0x0], 2),
    ([0xC2, 0xB2, 0x0], 2),
    ([0xC2, 0xB3, 0x0], 2),
    ([0xC2, 0xB4, 0x0], 2),
    ([0xC2, 0xB5, 0x0], 2),
    ([0xC2, 0xB6, 0x0], 2),
    ([0xC2, 0xB7, 0x0], 2),
    ([0xC2, 0xB8, 0x0], 2),
    ([0xC2, 0xB9, 0x0], 2),
    ([0xC2, 0xBA, 0x0], 2),
    ([0xC2, 0xBB, 0x0], 2),
    ([0xC2, 0xBC, 0x0], 2),
    ([0xC2, 0xBD, 0x0], 2),
    ([0xC2, 0xBE, 0x0], 2),
    ([0xC2, 0xBF, 0x0], 2),
    ([0xC3, 0x80, 0x0], 2),
    ([0xC3, 0x81, 0x0], 2),
    ([0xC3, 0x82, 0x0], 2),
    ([0xC3, 0x83, 0x0], 2),
    ([0xC3, 0x84, 0x0], 2),
    ([0xC3, 0x85, 0x0], 2),
    ([0xC3, 0x86, 0x0], 2),
    ([0xC3, 0x87, 0x0], 2),
    ([0xC3, 0x88, 0x0], 2),
    ([0xC3, 0x89, 0x0], 2),
    ([0xC3, 0x8A, 0x0], 2),
    ([0xC3, 0x8B, 0x0], 2),
    ([0xC3, 0x8C, 0x0], 2),
    ([0xC3, 0x8D, 0x0], 2),
    ([0xC3, 0x8E, 0x0], 2),
    ([0xC3, 0x8F, 0x0], 2),
    ([0xC3, 0x90, 0x0], 2),
    ([0xC3, 0x91, 0x0], 2),
    ([0xC3, 0x92, 0x0], 2),
    ([0xC3, 0x93, 0x0], 2),
    ([0xC3, 0x94, 0x0], 2),
    ([0xC3, 0x95, 0x0], 2),
    ([0xC3, 0x96, 0x0], 2),
    ([0xC3, 0x97, 0x0], 2),
    ([0xC3, 0x98, 0x0], 2),
    ([0xC3, 0x99, 0x0], 2),
    ([0xC3, 0x9A, 0x0], 2),
    ([0xC3, 0x9B, 0x0], 2),
    ([0xC3, 0x9C, 0x0], 2),
    ([0xC3, 0x9D, 0x0], 2),
    ([0xC3, 0x9E, 0x0], 2),
    ([0xC3, 0x9F, 0x0], 2),
    ([0xC3, 0xA0, 0x0], 2),
    ([0xC3, 0xA1, 0x0], 2),
    ([0xC3, 0xA2, 0x0], 2),
    ([0xC3, 0xA3, 0x0], 2),
    ([0xC3, 0xA4, 0x0], 2),
    ([0xC3, 0xA5, 0x0], 2),
    ([0xC3, 0xA6, 0x0], 2),
    ([0xC3, 0xA7, 0x0], 2),
    ([0xC3, 0xA8, 0x0], 2),
    ([0xC3, 0xA9, 0x0], 2),
    ([0xC3, 0xAA, 0x0], 2),
    ([0xC3, 0xAB, 0x0], 2),
    ([0xC3, 0xAC, 0x0], 2),
    ([0xC3, 0xAD, 0x0], 2),
    ([0xC3, 0xAE, 0x0], 2),
    ([0xC3, 0xAF, 0x0], 2),
    ([0xC3, 0xB0, 0x0], 2),
    ([0xC3, 0xB1, 0x0], 2),
    ([0xC3, 0xB2, 0x0], 2),
    ([0xC3, 0xB3, 0x0], 2),
    ([0xC3, 0xB4, 0x0], 2),
    ([0xC3, 0xB5, 0x0], 2),
    ([0xC3, 0xB6, 0x0], 2),
    ([0xC3, 0xB7, 0x0], 2),
    ([0xC3, 0xB8, 0x0], 2),
    ([0xC3, 0xB9, 0x0], 2),
    ([0xC3, 0xBA, 0x0], 2),
    ([0xC3, 0xBB, 0x0], 2),
    ([0xC3, 0xBC, 0x0], 2),
    ([0xC3, 0xBD, 0x0], 2),
    ([0xC3, 0xBE, 0x0], 2),
    ([0xC3, 0xBF, 0x0], 2),
];
impl Encoder for CP1252 {
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
                    0x81 => 0x81,
                    0x8D => 0x8D,
                    0x8F => 0x8F,
                    0x90 => 0x90,
                    0x9D => 0x9D,
                    0xA0 => 0xA0,
                    0xA1 => 0xA1,
                    0xA2 => 0xA2,
                    0xA3 => 0xA3,
                    0xA4 => 0xA4,
                    0xA5 => 0xA5,
                    0xA6 => 0xA6,
                    0xA7 => 0xA7,
                    0xA8 => 0xA8,
                    0xA9 => 0xA9,
                    0xAA => 0xAA,
                    0xAB => 0xAB,
                    0xAC => 0xAC,
                    0xAD => 0xAD,
                    0xAE => 0xAE,
                    0xAF => 0xAF,
                    0xB0 => 0xB0,
                    0xB1 => 0xB1,
                    0xB2 => 0xB2,
                    0xB3 => 0xB3,
                    0xB4 => 0xB4,
                    0xB5 => 0xB5,
                    0xB6 => 0xB6,
                    0xB7 => 0xB7,
                    0xB8 => 0xB8,
                    0xB9 => 0xB9,
                    0xBA => 0xBA,
                    0xBB => 0xBB,
                    0xBC => 0xBC,
                    0xBD => 0xBD,
                    0xBE => 0xBE,
                    0xBF => 0xBF,
                    _ => return None,
                }
            }
            (0xC3, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x80 => 0xC0,
                    0x81 => 0xC1,
                    0x82 => 0xC2,
                    0x83 => 0xC3,
                    0x84 => 0xC4,
                    0x85 => 0xC5,
                    0x86 => 0xC6,
                    0x87 => 0xC7,
                    0x88 => 0xC8,
                    0x89 => 0xC9,
                    0x8A => 0xCA,
                    0x8B => 0xCB,
                    0x8C => 0xCC,
                    0x8D => 0xCD,
                    0x8E => 0xCE,
                    0x8F => 0xCF,
                    0x90 => 0xD0,
                    0x91 => 0xD1,
                    0x92 => 0xD2,
                    0x93 => 0xD3,
                    0x94 => 0xD4,
                    0x95 => 0xD5,
                    0x96 => 0xD6,
                    0x97 => 0xD7,
                    0x98 => 0xD8,
                    0x99 => 0xD9,
                    0x9A => 0xDA,
                    0x9B => 0xDB,
                    0x9C => 0xDC,
                    0x9D => 0xDD,
                    0x9E => 0xDE,
                    0x9F => 0xDF,
                    0xA0 => 0xE0,
                    0xA1 => 0xE1,
                    0xA2 => 0xE2,
                    0xA3 => 0xE3,
                    0xA4 => 0xE4,
                    0xA5 => 0xE5,
                    0xA6 => 0xE6,
                    0xA7 => 0xE7,
                    0xA8 => 0xE8,
                    0xA9 => 0xE9,
                    0xAA => 0xEA,
                    0xAB => 0xEB,
                    0xAC => 0xEC,
                    0xAD => 0xED,
                    0xAE => 0xEE,
                    0xAF => 0xEF,
                    0xB0 => 0xF0,
                    0xB1 => 0xF1,
                    0xB2 => 0xF2,
                    0xB3 => 0xF3,
                    0xB4 => 0xF4,
                    0xB5 => 0xF5,
                    0xB6 => 0xF6,
                    0xB7 => 0xF7,
                    0xB8 => 0xF8,
                    0xB9 => 0xF9,
                    0xBA => 0xFA,
                    0xBB => 0xFB,
                    0xBC => 0xFC,
                    0xBD => 0xFD,
                    0xBE => 0xFE,
                    0xBF => 0xFF,
                    _ => return None,
                }
            }
            (0xC5, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0xA0 => 0x8A,
                    0x92 => 0x8C,
                    0xBD => 0x8E,
                    0xA1 => 0x9A,
                    0x93 => 0x9C,
                    0xBE => 0x9E,
                    0xB8 => 0x9F,
                    _ => return None,
                }
            }
            (0xC6, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x92 => 0x83,
                    _ => return None,
                }
            }
            (0xCB, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x86 => 0x88,
                    0x9C => 0x98,
                    _ => return None,
                }
            }
            (0xE2, [_, b, c, ..]) => {
                *bytes = &bytes[3..];
                match b {
                    0x80 => match c {
                        0x9A => 0x82,
                        0x9E => 0x84,
                        0xA6 => 0x85,
                        0xA0 => 0x86,
                        0xA1 => 0x87,
                        0xB0 => 0x89,
                        0xB9 => 0x8B,
                        0x98 => 0x91,
                        0x99 => 0x92,
                        0x9C => 0x93,
                        0x9D => 0x94,
                        0xA2 => 0x95,
                        0x93 => 0x96,
                        0x94 => 0x97,
                        0xBA => 0x9B,
                        _ => return None,
                    },
                    0x82 => match c {
                        0xAC => 0x80,
                        _ => return None,
                    },
                    0x84 => match c {
                        0xA2 => 0x99,
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
