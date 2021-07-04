// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/
// See binary codegen crate
use crate::internal::{DecoderIncomplete, Encoder, NZ_ONE, NZ_THREE, NZ_TWO};
use crate::{CodePage, DecodeError, EncodeError};
use std::borrow::Cow;
use std::num::NonZeroU8;

impl CP869 {
    /// Decode CP869 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will result in [`DecodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP869;
    ///
    /// assert_eq!(CP869.decode(&[116, 101, 120, 116]).unwrap(), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Result<Cow<str>, DecodeError> {
        Self::decode_helper(bytes, None)
    }

    /// Decode CP869 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `'�'`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP869;
    ///
    /// assert_eq!(CP869.decode_lossy(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy(self, bytes: &[u8]) -> Cow<str> {
        Self::decode_helper(bytes, Some('�')).unwrap()
    }

    /// Decode CP869 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `fallback` char.
    ///
    /// Note that the `fallback` char should be less than 4 bytes in UTF8, otherwise it will panic at the start of the function.
    /// Refrain from using emojis as fallback
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP869;
    ///
    /// assert_eq!(CP869.decode_lossy_fallback(&[116, 101, 120, 116], '�'), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy_fallback(self, bytes: &[u8], fallback: char) -> Cow<str> {
        Self::decode_helper(bytes, Some(fallback)).unwrap()
    }

    /// Encode UTF-8 string into CP869 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP869;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP869.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP869.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CP869 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP869;
    ///
    /// assert_eq!(CP869.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct CP869;

impl CodePage for CP869 {
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

impl DecoderIncomplete for CP869 {
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
        None,
        None,
        None,
        None,
        None,
        None,
        Some(([0xCE, 0x86, 0x0], NZ_TWO)),
        None,
        Some(([0xC2, 0xB7, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAC, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA6, 0x0], NZ_TWO)),
        Some(([0xE2, 0x80, 0x98], NZ_THREE)),
        Some(([0xE2, 0x80, 0x99], NZ_THREE)),
        Some(([0xCE, 0x88, 0x0], NZ_TWO)),
        Some(([0xE2, 0x80, 0x95], NZ_THREE)),
        Some(([0xCE, 0x89, 0x0], NZ_TWO)),
        Some(([0xCE, 0x8A, 0x0], NZ_TWO)),
        Some(([0xCE, 0xAA, 0x0], NZ_TWO)),
        Some(([0xCE, 0x8C, 0x0], NZ_TWO)),
        None,
        None,
        Some(([0xCE, 0x8E, 0x0], NZ_TWO)),
        Some(([0xCE, 0xAB, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA9, 0x0], NZ_TWO)),
        Some(([0xCE, 0x8F, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB2, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB3, 0x0], NZ_TWO)),
        Some(([0xCE, 0xAC, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA3, 0x0], NZ_TWO)),
        Some(([0xCE, 0xAD, 0x0], NZ_TWO)),
        Some(([0xCE, 0xAE, 0x0], NZ_TWO)),
        Some(([0xCE, 0xAF, 0x0], NZ_TWO)),
        Some(([0xCF, 0x8A, 0x0], NZ_TWO)),
        Some(([0xCE, 0x90, 0x0], NZ_TWO)),
        Some(([0xCF, 0x8C, 0x0], NZ_TWO)),
        Some(([0xCF, 0x8D, 0x0], NZ_TWO)),
        Some(([0xCE, 0x91, 0x0], NZ_TWO)),
        Some(([0xCE, 0x92, 0x0], NZ_TWO)),
        Some(([0xCE, 0x93, 0x0], NZ_TWO)),
        Some(([0xCE, 0x94, 0x0], NZ_TWO)),
        Some(([0xCE, 0x95, 0x0], NZ_TWO)),
        Some(([0xCE, 0x96, 0x0], NZ_TWO)),
        Some(([0xCE, 0x97, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBD, 0x0], NZ_TWO)),
        Some(([0xCE, 0x98, 0x0], NZ_TWO)),
        Some(([0xCE, 0x99, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAB, 0x0], NZ_TWO)),
        Some(([0xC2, 0xBB, 0x0], NZ_TWO)),
        Some(([0xE2, 0x96, 0x91], NZ_THREE)),
        Some(([0xE2, 0x96, 0x92], NZ_THREE)),
        Some(([0xE2, 0x96, 0x93], NZ_THREE)),
        Some(([0xE2, 0x94, 0x82], NZ_THREE)),
        Some(([0xE2, 0x94, 0xA4], NZ_THREE)),
        Some(([0xCE, 0x9A, 0x0], NZ_TWO)),
        Some(([0xCE, 0x9B, 0x0], NZ_TWO)),
        Some(([0xCE, 0x9C, 0x0], NZ_TWO)),
        Some(([0xCE, 0x9D, 0x0], NZ_TWO)),
        Some(([0xE2, 0x95, 0xA3], NZ_THREE)),
        Some(([0xE2, 0x95, 0x91], NZ_THREE)),
        Some(([0xE2, 0x95, 0x97], NZ_THREE)),
        Some(([0xE2, 0x95, 0x9D], NZ_THREE)),
        Some(([0xCE, 0x9E, 0x0], NZ_TWO)),
        Some(([0xCE, 0x9F, 0x0], NZ_TWO)),
        Some(([0xE2, 0x94, 0x90], NZ_THREE)),
        Some(([0xE2, 0x94, 0x94], NZ_THREE)),
        Some(([0xE2, 0x94, 0xB4], NZ_THREE)),
        Some(([0xE2, 0x94, 0xAC], NZ_THREE)),
        Some(([0xE2, 0x94, 0x9C], NZ_THREE)),
        Some(([0xE2, 0x94, 0x80], NZ_THREE)),
        Some(([0xE2, 0x94, 0xBC], NZ_THREE)),
        Some(([0xCE, 0xA0, 0x0], NZ_TWO)),
        Some(([0xCE, 0xA1, 0x0], NZ_TWO)),
        Some(([0xE2, 0x95, 0x9A], NZ_THREE)),
        Some(([0xE2, 0x95, 0x94], NZ_THREE)),
        Some(([0xE2, 0x95, 0xA9], NZ_THREE)),
        Some(([0xE2, 0x95, 0xA6], NZ_THREE)),
        Some(([0xE2, 0x95, 0xA0], NZ_THREE)),
        Some(([0xE2, 0x95, 0x90], NZ_THREE)),
        Some(([0xE2, 0x95, 0xAC], NZ_THREE)),
        Some(([0xCE, 0xA3, 0x0], NZ_TWO)),
        Some(([0xCE, 0xA4, 0x0], NZ_TWO)),
        Some(([0xCE, 0xA5, 0x0], NZ_TWO)),
        Some(([0xCE, 0xA6, 0x0], NZ_TWO)),
        Some(([0xCE, 0xA7, 0x0], NZ_TWO)),
        Some(([0xCE, 0xA8, 0x0], NZ_TWO)),
        Some(([0xCE, 0xA9, 0x0], NZ_TWO)),
        Some(([0xCE, 0xB1, 0x0], NZ_TWO)),
        Some(([0xCE, 0xB2, 0x0], NZ_TWO)),
        Some(([0xCE, 0xB3, 0x0], NZ_TWO)),
        Some(([0xE2, 0x94, 0x98], NZ_THREE)),
        Some(([0xE2, 0x94, 0x8C], NZ_THREE)),
        Some(([0xE2, 0x96, 0x88], NZ_THREE)),
        Some(([0xE2, 0x96, 0x84], NZ_THREE)),
        Some(([0xCE, 0xB4, 0x0], NZ_TWO)),
        Some(([0xCE, 0xB5, 0x0], NZ_TWO)),
        Some(([0xE2, 0x96, 0x80], NZ_THREE)),
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
        Some(([0xCF, 0x83, 0x0], NZ_TWO)),
        Some(([0xCF, 0x82, 0x0], NZ_TWO)),
        Some(([0xCF, 0x84, 0x0], NZ_TWO)),
        Some(([0xCE, 0x84, 0x0], NZ_TWO)),
        Some(([0xC2, 0xAD, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB1, 0x0], NZ_TWO)),
        Some(([0xCF, 0x85, 0x0], NZ_TWO)),
        Some(([0xCF, 0x86, 0x0], NZ_TWO)),
        Some(([0xCF, 0x87, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA7, 0x0], NZ_TWO)),
        Some(([0xCF, 0x88, 0x0], NZ_TWO)),
        Some(([0xCE, 0x85, 0x0], NZ_TWO)),
        Some(([0xC2, 0xB0, 0x0], NZ_TWO)),
        Some(([0xC2, 0xA8, 0x0], NZ_TWO)),
        Some(([0xCF, 0x89, 0x0], NZ_TWO)),
        Some(([0xCF, 0x8B, 0x0], NZ_TWO)),
        Some(([0xCE, 0xB0, 0x0], NZ_TWO)),
        Some(([0xCF, 0x8E, 0x0], NZ_TWO)),
        Some(([0xE2, 0x96, 0xA0], NZ_THREE)),
        Some(([0xC2, 0xA0, 0x0], NZ_TWO)),
    ];
}
impl Encoder for CP869 {
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
                    0xB7 => 0x88,
                    0xAC => 0x89,
                    0xA6 => 0x8A,
                    0xA9 => 0x97,
                    0xB2 => 0x99,
                    0xB3 => 0x9A,
                    0xA3 => 0x9C,
                    0xBD => 0xAB,
                    0xAB => 0xAE,
                    0xBB => 0xAF,
                    0xAD => 0xF0,
                    0xB1 => 0xF1,
                    0xA7 => 0xF5,
                    0xB0 => 0xF8,
                    0xA8 => 0xF9,
                    0xA0 => 0xFF,
                    _ => return None,
                }
            }
            (0xCE, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x86 => 0x86,
                    0x88 => 0x8D,
                    0x89 => 0x8F,
                    0x8A => 0x90,
                    0xAA => 0x91,
                    0x8C => 0x92,
                    0x8E => 0x95,
                    0xAB => 0x96,
                    0x8F => 0x98,
                    0xAC => 0x9B,
                    0xAD => 0x9D,
                    0xAE => 0x9E,
                    0xAF => 0x9F,
                    0x90 => 0xA1,
                    0x91 => 0xA4,
                    0x92 => 0xA5,
                    0x93 => 0xA6,
                    0x94 => 0xA7,
                    0x95 => 0xA8,
                    0x96 => 0xA9,
                    0x97 => 0xAA,
                    0x98 => 0xAC,
                    0x99 => 0xAD,
                    0x9A => 0xB5,
                    0x9B => 0xB6,
                    0x9C => 0xB7,
                    0x9D => 0xB8,
                    0x9E => 0xBD,
                    0x9F => 0xBE,
                    0xA0 => 0xC6,
                    0xA1 => 0xC7,
                    0xA3 => 0xCF,
                    0xA4 => 0xD0,
                    0xA5 => 0xD1,
                    0xA6 => 0xD2,
                    0xA7 => 0xD3,
                    0xA8 => 0xD4,
                    0xA9 => 0xD5,
                    0xB1 => 0xD6,
                    0xB2 => 0xD7,
                    0xB3 => 0xD8,
                    0xB4 => 0xDD,
                    0xB5 => 0xDE,
                    0xB6 => 0xE0,
                    0xB7 => 0xE1,
                    0xB8 => 0xE2,
                    0xB9 => 0xE3,
                    0xBA => 0xE4,
                    0xBB => 0xE5,
                    0xBC => 0xE6,
                    0xBD => 0xE7,
                    0xBE => 0xE8,
                    0xBF => 0xE9,
                    0x84 => 0xEF,
                    0x85 => 0xF7,
                    0xB0 => 0xFC,
                    _ => return None,
                }
            }
            (0xCF, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x8A => 0xA0,
                    0x8C => 0xA2,
                    0x8D => 0xA3,
                    0x80 => 0xEA,
                    0x81 => 0xEB,
                    0x83 => 0xEC,
                    0x82 => 0xED,
                    0x84 => 0xEE,
                    0x85 => 0xF2,
                    0x86 => 0xF3,
                    0x87 => 0xF4,
                    0x88 => 0xF6,
                    0x89 => 0xFA,
                    0x8B => 0xFB,
                    0x8E => 0xFD,
                    _ => return None,
                }
            }
            (0xE2, [_, b, c, ..]) => {
                *bytes = &bytes[3..];
                match b {
                    0x80 => match c {
                        0x98 => 0x8B,
                        0x99 => 0x8C,
                        0x95 => 0x8E,
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
                        0xA3 => 0xB9,
                        0x91 => 0xBA,
                        0x97 => 0xBB,
                        0x9D => 0xBC,
                        0x9A => 0xC8,
                        0x94 => 0xC9,
                        0xA9 => 0xCA,
                        0xA6 => 0xCB,
                        0xA0 => 0xCC,
                        0x90 => 0xCD,
                        0xAC => 0xCE,
                        _ => return None,
                    },
                    0x96 => match c {
                        0x91 => 0xB0,
                        0x92 => 0xB1,
                        0x93 => 0xB2,
                        0x88 => 0xDB,
                        0x84 => 0xDC,
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
