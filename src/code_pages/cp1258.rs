// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/
// See binary codegen crate
use crate::internal::{DecoderIncomplete, Encoder, NZ_ONE, NZ_THREE, NZ_TWO};
use crate::{CodePage, DecodeError, EncodeError};
use std::borrow::Cow;
use std::num::NonZeroU8;

impl CP1258 {
    /// Decode CP1258 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will result in [`DecodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1258;
    ///
    /// assert_eq!(CP1258.decode(&[116, 101, 120, 116]).unwrap(), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Result<Cow<str>, DecodeError> {
        Self::decode_helper(bytes, None)
    }

    /// Decode CP1258 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `'�'`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1258;
    ///
    /// assert_eq!(CP1258.decode_lossy(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy(self, bytes: &[u8]) -> Cow<str> {
        Self::decode_helper(bytes, Some('�')).unwrap()
    }

    /// Decode CP1258 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `fallback` char.
    ///
    /// Note that the `fallback` char should be less than 4 bytes in UTF8, otherwise it will panic at the start of the function.
    /// Refrain from using emojis as fallback
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1258;
    ///
    /// assert_eq!(CP1258.decode_lossy_fallback(&[116, 101, 120, 116], '�'), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy_fallback(self, bytes: &[u8], fallback: char) -> Cow<str> {
        Self::decode_helper(bytes, Some(fallback)).unwrap()
    }

    /// Encode UTF-8 string into CP1258 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1258;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP1258.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP1258.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CP1258 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1258;
    ///
    /// assert_eq!(CP1258.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct CP1258;

impl CodePage for CP1258 {
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

impl DecoderIncomplete for CP1258 {
    const DECODE_TABLE: [Option<([u8; 3], NonZeroU8)>; 256] = [
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
        None,
        Some(([0xE2, 0x80, 0x9A], NZ_THREE)),
        Some(([0xC6, 0x92, 0x0], NZ_TWO)),
        Some(([0xE2, 0x80, 0x9E], NZ_THREE)),
        Some(([0xE2, 0x80, 0xA6], NZ_THREE)),
        Some(([0xE2, 0x80, 0xA0], NZ_THREE)),
        Some(([0xE2, 0x80, 0xA1], NZ_THREE)),
        Some(([0xCB, 0x86, 0x0], NZ_TWO)),
        Some(([0xE2, 0x80, 0xB0], NZ_THREE)),
        None,
        Some(([0xE2, 0x80, 0xB9], NZ_THREE)),
        Some(([0xC5, 0x92, 0x0], NZ_TWO)),
        None,
        None,
        None,
        None,
        Some(([0xE2, 0x80, 0x98], NZ_THREE)),
        Some(([0xE2, 0x80, 0x99], NZ_THREE)),
        Some(([0xE2, 0x80, 0x9C], NZ_THREE)),
        Some(([0xE2, 0x80, 0x9D], NZ_THREE)),
        Some(([0xE2, 0x80, 0xA2], NZ_THREE)),
        Some(([0xE2, 0x80, 0x93], NZ_THREE)),
        Some(([0xE2, 0x80, 0x94], NZ_THREE)),
        Some(([0xCB, 0x9C, 0x0], NZ_TWO)),
        Some(([0xE2, 0x84, 0xA2], NZ_THREE)),
        None,
        Some(([0xE2, 0x80, 0xBA], NZ_THREE)),
        Some(([0xC5, 0x93, 0x0], NZ_TWO)),
        None,
        None,
        Some(([0xC5, 0xB8, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA0, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA1, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA2, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA3, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA4, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA5, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA6, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA7, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA8, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA9, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAA, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAB, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAC, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAD, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAE, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAF, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB0, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB1, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB2, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB3, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB4, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB5, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB6, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB7, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB8, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB9, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBA, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBB, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBC, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBD, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBE, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBF, 0x0], NZ_TWO)),
        Some(([0xC3, 0x80, 0x0], NZ_TWO)),
        Some(([0xC3, 0x81, 0x0], NZ_TWO)),
        Some(([0xC3, 0x82, 0x0], NZ_TWO)),
        Some(([0xC4, 0x82, 0x0], NZ_TWO)),
        Some(([0xC3, 0x84, 0x0], NZ_TWO)),
        Some(([0xC3, 0x85, 0x0], NZ_TWO)),
        Some(([0xC3, 0x86, 0x0], NZ_TWO)),
        Some(([0xC3, 0x87, 0x0], NZ_TWO)),
        Some(([0xC3, 0x88, 0x0], NZ_TWO)),
        Some(([0xC3, 0x89, 0x0], NZ_TWO)),
        Some(([0xC3, 0x8A, 0x0], NZ_TWO)),
        Some(([0xC3, 0x8B, 0x0], NZ_TWO)),
        Some(([0xCC, 0x80, 0x0], NZ_TWO)),
        Some(([0xC3, 0x8D, 0x0], NZ_TWO)),
        Some(([0xC3, 0x8E, 0x0], NZ_TWO)),
        Some(([0xC3, 0x8F, 0x0], NZ_TWO)),
        Some(([0xC4, 0x90, 0x0], NZ_TWO)),
        Some(([0xC3, 0x91, 0x0], NZ_TWO)),
        Some(([0xCC, 0x89, 0x0], NZ_TWO)),
        Some(([0xC3, 0x93, 0x0], NZ_TWO)),
        Some(([0xC3, 0x94, 0x0], NZ_TWO)),
        Some(([0xC6, 0xA0, 0x0], NZ_TWO)),
        Some(([0xC3, 0x96, 0x0], NZ_TWO)),
        Some(([0xC3, 0x97, 0x0], NZ_TWO)),
        Some(([0xC3, 0x98, 0x0], NZ_TWO)),
        Some(([0xC3, 0x99, 0x0], NZ_TWO)),
        Some(([0xC3, 0x9A, 0x0], NZ_TWO)),
        Some(([0xC3, 0x9B, 0x0], NZ_TWO)),
        Some(([0xC3, 0x9C, 0x0], NZ_TWO)),
        Some(([0xC6, 0xAF, 0x0], NZ_TWO)),
        Some(([0xCC, 0x83, 0x0], NZ_TWO)),
        Some(([0xC3, 0x9F, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA0, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA1, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA2, 0x0], NZ_TWO)),
        Some(([0xC4, 0x83, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA4, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA5, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA6, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA7, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA8, 0x0], NZ_TWO)),
        Some(([0xC3, 0xA9, 0x0], NZ_TWO)),
        Some(([0xC3, 0xAA, 0x0], NZ_TWO)),
        Some(([0xC3, 0xAB, 0x0], NZ_TWO)),
        Some(([0xCC, 0x81, 0x0], NZ_TWO)),
        Some(([0xC3, 0xAD, 0x0], NZ_TWO)),
        Some(([0xC3, 0xAE, 0x0], NZ_TWO)),
        Some(([0xC3, 0xAF, 0x0], NZ_TWO)),
        Some(([0xC4, 0x91, 0x0], NZ_TWO)),
        Some(([0xC3, 0xB1, 0x0], NZ_TWO)),
        Some(([0xCC, 0xA3, 0x0], NZ_TWO)),
        Some(([0xC3, 0xB3, 0x0], NZ_TWO)),
        Some(([0xC3, 0xB4, 0x0], NZ_TWO)),
        Some(([0xC6, 0xA1, 0x0], NZ_TWO)),
        Some(([0xC3, 0xB6, 0x0], NZ_TWO)),
        Some(([0xC3, 0xB7, 0x0], NZ_TWO)),
        Some(([0xC3, 0xB8, 0x0], NZ_TWO)),
        Some(([0xC3, 0xB9, 0x0], NZ_TWO)),
        Some(([0xC3, 0xBA, 0x0], NZ_TWO)),
        Some(([0xC3, 0xBB, 0x0], NZ_TWO)),
        Some(([0xC3, 0xBC, 0x0], NZ_TWO)),
        Some(([0xC6, 0xB0, 0x0], NZ_TWO)),
        Some(([0xE2, 0x82, 0xAB], NZ_THREE)),
        Some(([0xC3, 0xBF, 0x0], NZ_TWO)),
    ];
}
impl Encoder for CP1258 {
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
                    0x84 => 0xC4,
                    0x85 => 0xC5,
                    0x86 => 0xC6,
                    0x87 => 0xC7,
                    0x88 => 0xC8,
                    0x89 => 0xC9,
                    0x8A => 0xCA,
                    0x8B => 0xCB,
                    0x8D => 0xCD,
                    0x8E => 0xCE,
                    0x8F => 0xCF,
                    0x91 => 0xD1,
                    0x93 => 0xD3,
                    0x94 => 0xD4,
                    0x96 => 0xD6,
                    0x97 => 0xD7,
                    0x98 => 0xD8,
                    0x99 => 0xD9,
                    0x9A => 0xDA,
                    0x9B => 0xDB,
                    0x9C => 0xDC,
                    0x9F => 0xDF,
                    0xA0 => 0xE0,
                    0xA1 => 0xE1,
                    0xA2 => 0xE2,
                    0xA4 => 0xE4,
                    0xA5 => 0xE5,
                    0xA6 => 0xE6,
                    0xA7 => 0xE7,
                    0xA8 => 0xE8,
                    0xA9 => 0xE9,
                    0xAA => 0xEA,
                    0xAB => 0xEB,
                    0xAD => 0xED,
                    0xAE => 0xEE,
                    0xAF => 0xEF,
                    0xB1 => 0xF1,
                    0xB3 => 0xF3,
                    0xB4 => 0xF4,
                    0xB6 => 0xF6,
                    0xB7 => 0xF7,
                    0xB8 => 0xF8,
                    0xB9 => 0xF9,
                    0xBA => 0xFA,
                    0xBB => 0xFB,
                    0xBC => 0xFC,
                    0xBF => 0xFF,
                    _ => return None,
                }
            }
            (0xC4, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x82 => 0xC3,
                    0x90 => 0xD0,
                    0x83 => 0xE3,
                    0x91 => 0xF0,
                    _ => return None,
                }
            }
            (0xC5, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x92 => 0x8C,
                    0x93 => 0x9C,
                    0xB8 => 0x9F,
                    _ => return None,
                }
            }
            (0xC6, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x92 => 0x83,
                    0xA0 => 0xD5,
                    0xAF => 0xDD,
                    0xA1 => 0xF5,
                    0xB0 => 0xFD,
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
            (0xCC, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x80 => 0xCC,
                    0x89 => 0xD2,
                    0x83 => 0xDE,
                    0x81 => 0xEC,
                    0xA3 => 0xF2,
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
                        0xAB => 0xFE,
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
