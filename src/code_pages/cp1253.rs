// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/
// See binary codegen crate
use crate::internal::decoder_incomplete::{decode_helper, NZ_ONE, NZ_THREE, NZ_TWO};
use crate::internal::Encoder;
use crate::{CodePage, DecodeError, EncodeError};
use std::borrow::Cow;

impl CP1253 {
    /// Decode CP1253 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will result in [`DecodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1253;
    ///
    /// assert_eq!(CP1253.decode(&[116, 101, 120, 116]).unwrap(), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Result<Cow<str>, DecodeError> {
        decode_helper(&DECODE_TABLE, bytes, None)
    }

    /// Decode CP1253 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `'�'`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1253;
    ///
    /// assert_eq!(CP1253.decode_lossy(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy(self, bytes: &[u8]) -> Cow<str> {
        decode_helper(&DECODE_TABLE, bytes, Some('�')).unwrap()
    }

    /// Decode CP1253 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `fallback` char.
    ///
    /// Note that the `fallback` char should be less than 4 bytes in UTF8, otherwise it will panic at the start of the function.
    /// Refrain from using emojis as fallback
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1253;
    ///
    /// assert_eq!(CP1253.decode_lossy_fallback(&[116, 101, 120, 116], '�'), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy_fallback(self, bytes: &[u8], fallback: char) -> Cow<str> {
        decode_helper(&DECODE_TABLE, bytes, Some(fallback)).unwrap()
    }

    /// Encode UTF-8 string into CP1253 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1253;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP1253.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP1253.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CP1253 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1253;
    ///
    /// assert_eq!(CP1253.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct CP1253;

impl CodePage for CP1253 {
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        (*self).decode(bytes)
    }

    #[inline(always)]
    fn decode_lossy<'a>(&self, bytes: &'a [u8]) -> Cow<'a, str> {
        (*self).decode_lossy(bytes)
    }

    /// Note that the `fallback` char should be less than 4 bytes in UTF8.
    /// 4 bytes UTF8 will panic because of assertion.
    /// Refrain from using emojis as fallback
    #[inline(always)]
    fn decode_lossy_fallback<'a>(&self, bytes: &'a [u8], fallback: char) -> Cow<'a, str> {
        (*self).decode_lossy_fallback(bytes, fallback)
    }
}

const DECODE_TABLE: crate::internal::decoder_incomplete::Table = [
    Some(([0x0, 0x0, 0x0], NZ_ONE)),
    Some(([0x1, 0x0, 0x0], NZ_ONE)),
    Some(([0x2, 0x0, 0x0], NZ_ONE)),
    Some(([0x3, 0x0, 0x0], NZ_ONE)),
    Some(([0x4, 0x0, 0x0], NZ_ONE)),
    Some(([0x5, 0x0, 0x0], NZ_ONE)),
    Some(([0x6, 0x0, 0x0], NZ_ONE)),
    Some(([0x7, 0x0, 0x0], NZ_ONE)),
    Some(([0x8, 0x0, 0x0], NZ_ONE)),
    Some(([0x9, 0x0, 0x0], NZ_ONE)),
    Some(([0xA, 0x0, 0x0], NZ_ONE)),
    Some(([0xB, 0x0, 0x0], NZ_ONE)),
    Some(([0xC, 0x0, 0x0], NZ_ONE)),
    Some(([0xD, 0x0, 0x0], NZ_ONE)),
    Some(([0xE, 0x0, 0x0], NZ_ONE)),
    Some(([0xF, 0x0, 0x0], NZ_ONE)),
    Some(([0x10, 0x0, 0x0], NZ_ONE)),
    Some(([0x11, 0x0, 0x0], NZ_ONE)),
    Some(([0x12, 0x0, 0x0], NZ_ONE)),
    Some(([0x13, 0x0, 0x0], NZ_ONE)),
    Some(([0x14, 0x0, 0x0], NZ_ONE)),
    Some(([0x15, 0x0, 0x0], NZ_ONE)),
    Some(([0x16, 0x0, 0x0], NZ_ONE)),
    Some(([0x17, 0x0, 0x0], NZ_ONE)),
    Some(([0x18, 0x0, 0x0], NZ_ONE)),
    Some(([0x19, 0x0, 0x0], NZ_ONE)),
    Some(([0x1A, 0x0, 0x0], NZ_ONE)),
    Some(([0x1B, 0x0, 0x0], NZ_ONE)),
    Some(([0x1C, 0x0, 0x0], NZ_ONE)),
    Some(([0x1D, 0x0, 0x0], NZ_ONE)),
    Some(([0x1E, 0x0, 0x0], NZ_ONE)),
    Some(([0x1F, 0x0, 0x0], NZ_ONE)),
    Some(([0x20, 0x0, 0x0], NZ_ONE)),
    Some(([0x21, 0x0, 0x0], NZ_ONE)),
    Some(([0x22, 0x0, 0x0], NZ_ONE)),
    Some(([0x23, 0x0, 0x0], NZ_ONE)),
    Some(([0x24, 0x0, 0x0], NZ_ONE)),
    Some(([0x25, 0x0, 0x0], NZ_ONE)),
    Some(([0x26, 0x0, 0x0], NZ_ONE)),
    Some(([0x27, 0x0, 0x0], NZ_ONE)),
    Some(([0x28, 0x0, 0x0], NZ_ONE)),
    Some(([0x29, 0x0, 0x0], NZ_ONE)),
    Some(([0x2A, 0x0, 0x0], NZ_ONE)),
    Some(([0x2B, 0x0, 0x0], NZ_ONE)),
    Some(([0x2C, 0x0, 0x0], NZ_ONE)),
    Some(([0x2D, 0x0, 0x0], NZ_ONE)),
    Some(([0x2E, 0x0, 0x0], NZ_ONE)),
    Some(([0x2F, 0x0, 0x0], NZ_ONE)),
    Some(([0x30, 0x0, 0x0], NZ_ONE)),
    Some(([0x31, 0x0, 0x0], NZ_ONE)),
    Some(([0x32, 0x0, 0x0], NZ_ONE)),
    Some(([0x33, 0x0, 0x0], NZ_ONE)),
    Some(([0x34, 0x0, 0x0], NZ_ONE)),
    Some(([0x35, 0x0, 0x0], NZ_ONE)),
    Some(([0x36, 0x0, 0x0], NZ_ONE)),
    Some(([0x37, 0x0, 0x0], NZ_ONE)),
    Some(([0x38, 0x0, 0x0], NZ_ONE)),
    Some(([0x39, 0x0, 0x0], NZ_ONE)),
    Some(([0x3A, 0x0, 0x0], NZ_ONE)),
    Some(([0x3B, 0x0, 0x0], NZ_ONE)),
    Some(([0x3C, 0x0, 0x0], NZ_ONE)),
    Some(([0x3D, 0x0, 0x0], NZ_ONE)),
    Some(([0x3E, 0x0, 0x0], NZ_ONE)),
    Some(([0x3F, 0x0, 0x0], NZ_ONE)),
    Some(([0x40, 0x0, 0x0], NZ_ONE)),
    Some(([0x41, 0x0, 0x0], NZ_ONE)),
    Some(([0x42, 0x0, 0x0], NZ_ONE)),
    Some(([0x43, 0x0, 0x0], NZ_ONE)),
    Some(([0x44, 0x0, 0x0], NZ_ONE)),
    Some(([0x45, 0x0, 0x0], NZ_ONE)),
    Some(([0x46, 0x0, 0x0], NZ_ONE)),
    Some(([0x47, 0x0, 0x0], NZ_ONE)),
    Some(([0x48, 0x0, 0x0], NZ_ONE)),
    Some(([0x49, 0x0, 0x0], NZ_ONE)),
    Some(([0x4A, 0x0, 0x0], NZ_ONE)),
    Some(([0x4B, 0x0, 0x0], NZ_ONE)),
    Some(([0x4C, 0x0, 0x0], NZ_ONE)),
    Some(([0x4D, 0x0, 0x0], NZ_ONE)),
    Some(([0x4E, 0x0, 0x0], NZ_ONE)),
    Some(([0x4F, 0x0, 0x0], NZ_ONE)),
    Some(([0x50, 0x0, 0x0], NZ_ONE)),
    Some(([0x51, 0x0, 0x0], NZ_ONE)),
    Some(([0x52, 0x0, 0x0], NZ_ONE)),
    Some(([0x53, 0x0, 0x0], NZ_ONE)),
    Some(([0x54, 0x0, 0x0], NZ_ONE)),
    Some(([0x55, 0x0, 0x0], NZ_ONE)),
    Some(([0x56, 0x0, 0x0], NZ_ONE)),
    Some(([0x57, 0x0, 0x0], NZ_ONE)),
    Some(([0x58, 0x0, 0x0], NZ_ONE)),
    Some(([0x59, 0x0, 0x0], NZ_ONE)),
    Some(([0x5A, 0x0, 0x0], NZ_ONE)),
    Some(([0x5B, 0x0, 0x0], NZ_ONE)),
    Some(([0x5C, 0x0, 0x0], NZ_ONE)),
    Some(([0x5D, 0x0, 0x0], NZ_ONE)),
    Some(([0x5E, 0x0, 0x0], NZ_ONE)),
    Some(([0x5F, 0x0, 0x0], NZ_ONE)),
    Some(([0x60, 0x0, 0x0], NZ_ONE)),
    Some(([0x61, 0x0, 0x0], NZ_ONE)),
    Some(([0x62, 0x0, 0x0], NZ_ONE)),
    Some(([0x63, 0x0, 0x0], NZ_ONE)),
    Some(([0x64, 0x0, 0x0], NZ_ONE)),
    Some(([0x65, 0x0, 0x0], NZ_ONE)),
    Some(([0x66, 0x0, 0x0], NZ_ONE)),
    Some(([0x67, 0x0, 0x0], NZ_ONE)),
    Some(([0x68, 0x0, 0x0], NZ_ONE)),
    Some(([0x69, 0x0, 0x0], NZ_ONE)),
    Some(([0x6A, 0x0, 0x0], NZ_ONE)),
    Some(([0x6B, 0x0, 0x0], NZ_ONE)),
    Some(([0x6C, 0x0, 0x0], NZ_ONE)),
    Some(([0x6D, 0x0, 0x0], NZ_ONE)),
    Some(([0x6E, 0x0, 0x0], NZ_ONE)),
    Some(([0x6F, 0x0, 0x0], NZ_ONE)),
    Some(([0x70, 0x0, 0x0], NZ_ONE)),
    Some(([0x71, 0x0, 0x0], NZ_ONE)),
    Some(([0x72, 0x0, 0x0], NZ_ONE)),
    Some(([0x73, 0x0, 0x0], NZ_ONE)),
    Some(([0x74, 0x0, 0x0], NZ_ONE)),
    Some(([0x75, 0x0, 0x0], NZ_ONE)),
    Some(([0x76, 0x0, 0x0], NZ_ONE)),
    Some(([0x77, 0x0, 0x0], NZ_ONE)),
    Some(([0x78, 0x0, 0x0], NZ_ONE)),
    Some(([0x79, 0x0, 0x0], NZ_ONE)),
    Some(([0x7A, 0x0, 0x0], NZ_ONE)),
    Some(([0x7B, 0x0, 0x0], NZ_ONE)),
    Some(([0x7C, 0x0, 0x0], NZ_ONE)),
    Some(([0x7D, 0x0, 0x0], NZ_ONE)),
    Some(([0x7E, 0x0, 0x0], NZ_ONE)),
    Some(([0x7F, 0x0, 0x0], NZ_ONE)),
    Some(([0xE2, 0x82, 0xAC], NZ_THREE)),
    Some(([0xC2, 0x81, 0x0], NZ_TWO)),
    Some(([0xE2, 0x80, 0x9A], NZ_THREE)),
    Some(([0xC6, 0x92, 0x0], NZ_TWO)),
    Some(([0xE2, 0x80, 0x9E], NZ_THREE)),
    Some(([0xE2, 0x80, 0xA6], NZ_THREE)),
    Some(([0xE2, 0x80, 0xA0], NZ_THREE)),
    Some(([0xE2, 0x80, 0xA1], NZ_THREE)),
    Some(([0xC2, 0x88, 0x0], NZ_TWO)),
    Some(([0xE2, 0x80, 0xB0], NZ_THREE)),
    Some(([0xC2, 0x8A, 0x0], NZ_TWO)),
    Some(([0xE2, 0x80, 0xB9], NZ_THREE)),
    Some(([0xC2, 0x8C, 0x0], NZ_TWO)),
    Some(([0xC2, 0x8D, 0x0], NZ_TWO)),
    Some(([0xC2, 0x8E, 0x0], NZ_TWO)),
    Some(([0xC2, 0x8F, 0x0], NZ_TWO)),
    Some(([0xC2, 0x90, 0x0], NZ_TWO)),
    Some(([0xE2, 0x80, 0x98], NZ_THREE)),
    Some(([0xE2, 0x80, 0x99], NZ_THREE)),
    Some(([0xE2, 0x80, 0x9C], NZ_THREE)),
    Some(([0xE2, 0x80, 0x9D], NZ_THREE)),
    Some(([0xE2, 0x80, 0xA2], NZ_THREE)),
    Some(([0xE2, 0x80, 0x93], NZ_THREE)),
    Some(([0xE2, 0x80, 0x94], NZ_THREE)),
    Some(([0xC2, 0x98, 0x0], NZ_TWO)),
    Some(([0xE2, 0x84, 0xA2], NZ_THREE)),
    Some(([0xC2, 0x9A, 0x0], NZ_TWO)),
    Some(([0xE2, 0x80, 0xBA], NZ_THREE)),
    Some(([0xC2, 0x9C, 0x0], NZ_TWO)),
    Some(([0xC2, 0x9D, 0x0], NZ_TWO)),
    Some(([0xC2, 0x9E, 0x0], NZ_TWO)),
    Some(([0xC2, 0x9F, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA0, 0x0], NZ_TWO)),
    Some(([0xCE, 0x85, 0x0], NZ_TWO)),
    Some(([0xCE, 0x86, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA3, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA4, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA5, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA6, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA7, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA8, 0x0], NZ_TWO)),
    Some(([0xC2, 0xA9, 0x0], NZ_TWO)),
    None,
    Some(([0xC2, 0xAB, 0x0], NZ_TWO)),
    Some(([0xC2, 0xAC, 0x0], NZ_TWO)),
    Some(([0xC2, 0xAD, 0x0], NZ_TWO)),
    Some(([0xC2, 0xAE, 0x0], NZ_TWO)),
    Some(([0xE2, 0x80, 0x95], NZ_THREE)),
    Some(([0xC2, 0xB0, 0x0], NZ_TWO)),
    Some(([0xC2, 0xB1, 0x0], NZ_TWO)),
    Some(([0xC2, 0xB2, 0x0], NZ_TWO)),
    Some(([0xC2, 0xB3, 0x0], NZ_TWO)),
    Some(([0xCE, 0x84, 0x0], NZ_TWO)),
    Some(([0xC2, 0xB5, 0x0], NZ_TWO)),
    Some(([0xC2, 0xB6, 0x0], NZ_TWO)),
    Some(([0xC2, 0xB7, 0x0], NZ_TWO)),
    Some(([0xCE, 0x88, 0x0], NZ_TWO)),
    Some(([0xCE, 0x89, 0x0], NZ_TWO)),
    Some(([0xCE, 0x8A, 0x0], NZ_TWO)),
    Some(([0xC2, 0xBB, 0x0], NZ_TWO)),
    Some(([0xCE, 0x8C, 0x0], NZ_TWO)),
    Some(([0xC2, 0xBD, 0x0], NZ_TWO)),
    Some(([0xCE, 0x8E, 0x0], NZ_TWO)),
    Some(([0xCE, 0x8F, 0x0], NZ_TWO)),
    Some(([0xCE, 0x90, 0x0], NZ_TWO)),
    Some(([0xCE, 0x91, 0x0], NZ_TWO)),
    Some(([0xCE, 0x92, 0x0], NZ_TWO)),
    Some(([0xCE, 0x93, 0x0], NZ_TWO)),
    Some(([0xCE, 0x94, 0x0], NZ_TWO)),
    Some(([0xCE, 0x95, 0x0], NZ_TWO)),
    Some(([0xCE, 0x96, 0x0], NZ_TWO)),
    Some(([0xCE, 0x97, 0x0], NZ_TWO)),
    Some(([0xCE, 0x98, 0x0], NZ_TWO)),
    Some(([0xCE, 0x99, 0x0], NZ_TWO)),
    Some(([0xCE, 0x9A, 0x0], NZ_TWO)),
    Some(([0xCE, 0x9B, 0x0], NZ_TWO)),
    Some(([0xCE, 0x9C, 0x0], NZ_TWO)),
    Some(([0xCE, 0x9D, 0x0], NZ_TWO)),
    Some(([0xCE, 0x9E, 0x0], NZ_TWO)),
    Some(([0xCE, 0x9F, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA0, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA1, 0x0], NZ_TWO)),
    None,
    Some(([0xCE, 0xA3, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA4, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA5, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA6, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA7, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA8, 0x0], NZ_TWO)),
    Some(([0xCE, 0xA9, 0x0], NZ_TWO)),
    Some(([0xCE, 0xAA, 0x0], NZ_TWO)),
    Some(([0xCE, 0xAB, 0x0], NZ_TWO)),
    Some(([0xCE, 0xAC, 0x0], NZ_TWO)),
    Some(([0xCE, 0xAD, 0x0], NZ_TWO)),
    Some(([0xCE, 0xAE, 0x0], NZ_TWO)),
    Some(([0xCE, 0xAF, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB0, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB1, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB2, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB3, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB4, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB5, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB6, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB7, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB8, 0x0], NZ_TWO)),
    Some(([0xCE, 0xB9, 0x0], NZ_TWO)),
    Some(([0xCE, 0xBA, 0x0], NZ_TWO)),
    Some(([0xCE, 0xBB, 0x0], NZ_TWO)),
    Some(([0xCE, 0xBC, 0x0], NZ_TWO)),
    Some(([0xCE, 0xBD, 0x0], NZ_TWO)),
    Some(([0xCE, 0xBE, 0x0], NZ_TWO)),
    Some(([0xCE, 0xBF, 0x0], NZ_TWO)),
    Some(([0xCF, 0x80, 0x0], NZ_TWO)),
    Some(([0xCF, 0x81, 0x0], NZ_TWO)),
    Some(([0xCF, 0x82, 0x0], NZ_TWO)),
    Some(([0xCF, 0x83, 0x0], NZ_TWO)),
    Some(([0xCF, 0x84, 0x0], NZ_TWO)),
    Some(([0xCF, 0x85, 0x0], NZ_TWO)),
    Some(([0xCF, 0x86, 0x0], NZ_TWO)),
    Some(([0xCF, 0x87, 0x0], NZ_TWO)),
    Some(([0xCF, 0x88, 0x0], NZ_TWO)),
    Some(([0xCF, 0x89, 0x0], NZ_TWO)),
    Some(([0xCF, 0x8A, 0x0], NZ_TWO)),
    Some(([0xCF, 0x8B, 0x0], NZ_TWO)),
    Some(([0xCF, 0x8C, 0x0], NZ_TWO)),
    Some(([0xCF, 0x8D, 0x0], NZ_TWO)),
    Some(([0xCF, 0x8E, 0x0], NZ_TWO)),
    None,
];
impl Encoder for CP1253 {
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
                    0x88 => 0x88,
                    0x8A => 0x8A,
                    0x8C => 0x8C,
                    0x8D => 0x8D,
                    0x8E => 0x8E,
                    0x8F => 0x8F,
                    0x90 => 0x90,
                    0x98 => 0x98,
                    0x9A => 0x9A,
                    0x9C => 0x9C,
                    0x9D => 0x9D,
                    0x9E => 0x9E,
                    0x9F => 0x9F,
                    0xA0 => 0xA0,
                    0xA3 => 0xA3,
                    0xA4 => 0xA4,
                    0xA5 => 0xA5,
                    0xA6 => 0xA6,
                    0xA7 => 0xA7,
                    0xA8 => 0xA8,
                    0xA9 => 0xA9,
                    0xAB => 0xAB,
                    0xAC => 0xAC,
                    0xAD => 0xAD,
                    0xAE => 0xAE,
                    0xB0 => 0xB0,
                    0xB1 => 0xB1,
                    0xB2 => 0xB2,
                    0xB3 => 0xB3,
                    0xB5 => 0xB5,
                    0xB6 => 0xB6,
                    0xB7 => 0xB7,
                    0xBB => 0xBB,
                    0xBD => 0xBD,
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
            (0xCE, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x85 => 0xA1,
                    0x86 => 0xA2,
                    0x84 => 0xB4,
                    0x88 => 0xB8,
                    0x89 => 0xB9,
                    0x8A => 0xBA,
                    0x8C => 0xBC,
                    0x8E => 0xBE,
                    0x8F => 0xBF,
                    0x90 => 0xC0,
                    0x91 => 0xC1,
                    0x92 => 0xC2,
                    0x93 => 0xC3,
                    0x94 => 0xC4,
                    0x95 => 0xC5,
                    0x96 => 0xC6,
                    0x97 => 0xC7,
                    0x98 => 0xC8,
                    0x99 => 0xC9,
                    0x9A => 0xCA,
                    0x9B => 0xCB,
                    0x9C => 0xCC,
                    0x9D => 0xCD,
                    0x9E => 0xCE,
                    0x9F => 0xCF,
                    0xA0 => 0xD0,
                    0xA1 => 0xD1,
                    0xA3 => 0xD3,
                    0xA4 => 0xD4,
                    0xA5 => 0xD5,
                    0xA6 => 0xD6,
                    0xA7 => 0xD7,
                    0xA8 => 0xD8,
                    0xA9 => 0xD9,
                    0xAA => 0xDA,
                    0xAB => 0xDB,
                    0xAC => 0xDC,
                    0xAD => 0xDD,
                    0xAE => 0xDE,
                    0xAF => 0xDF,
                    0xB0 => 0xE0,
                    0xB1 => 0xE1,
                    0xB2 => 0xE2,
                    0xB3 => 0xE3,
                    0xB4 => 0xE4,
                    0xB5 => 0xE5,
                    0xB6 => 0xE6,
                    0xB7 => 0xE7,
                    0xB8 => 0xE8,
                    0xB9 => 0xE9,
                    0xBA => 0xEA,
                    0xBB => 0xEB,
                    0xBC => 0xEC,
                    0xBD => 0xED,
                    0xBE => 0xEE,
                    0xBF => 0xEF,
                    _ => return None,
                }
            }
            (0xCF, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x80 => 0xF0,
                    0x81 => 0xF1,
                    0x82 => 0xF2,
                    0x83 => 0xF3,
                    0x84 => 0xF4,
                    0x85 => 0xF5,
                    0x86 => 0xF6,
                    0x87 => 0xF7,
                    0x88 => 0xF8,
                    0x89 => 0xF9,
                    0x8A => 0xFA,
                    0x8B => 0xFB,
                    0x8C => 0xFC,
                    0x8D => 0xFD,
                    0x8E => 0xFE,
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
                        0x95 => 0xAF,
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
