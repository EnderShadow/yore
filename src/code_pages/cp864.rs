// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/
// See binary codegen crate
use crate::internal::{DecoderIncomplete, Encoder};
use crate::{CodePage, DecodeError, EncodeError};
use std::borrow::Cow;
use std::num::NonZeroU8;

impl CP864 {
    /// Decode CP864 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will result in [`DecodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP864;
    ///
    /// assert_eq!(CP864.decode(&[116, 101, 120, 116]).unwrap(), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Result<Cow<str>, DecodeError> {
        Self::decode_helper(bytes, None)
    }

    /// Decode CP864 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `'�'`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP864;
    ///
    /// assert_eq!(CP864.decode_lossy(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy(self, bytes: &[u8]) -> Cow<str> {
        Self::decode_helper(bytes, Some('�')).unwrap()
    }

    /// Decode CP864 byte-encoding into UTF-8 string
    ///
    /// Undefined codepoints will be replaced with `fallback` char.
    ///
    /// Note that the `fallback` char should be less than 4 bytes in UTF8, otherwise it will panic at the start of the function.
    /// Refrain from using emojis as fallback
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP864;
    ///
    /// assert_eq!(CP864.decode_lossy_fallback(&[116, 101, 120, 116], '�'), "text");
    /// ```
    #[inline(always)]
    pub fn decode_lossy_fallback(self, bytes: &[u8], fallback: char) -> Cow<str> {
        Self::decode_helper(bytes, Some(fallback)).unwrap()
    }

    /// Encode UTF-8 string into CP864 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP864;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP864.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP864.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CP864 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP864;
    ///
    /// assert_eq!(CP864.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct CP864;

impl CodePage for CP864 {
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

impl DecoderIncomplete for CP864 {
    const DECODE_TABLE: [Option<([u8; 3], NonZeroU8)>; 256] = unsafe {
        [
            Some(([0x0, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x1, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x2, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x3, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x4, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x5, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x6, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x7, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x8, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x9, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xA, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xB, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xC, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xD, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xE, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xF, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x10, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x11, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x12, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x13, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x14, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x15, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x16, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x17, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x18, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x19, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x1A, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x1B, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x1C, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x1D, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x1E, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x1F, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x20, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x21, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x22, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x23, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x24, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xD9, 0xAA, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0x26, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x27, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x28, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x29, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x2A, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x2B, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x2C, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x2D, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x2E, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x2F, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x30, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x31, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x32, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x33, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x34, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x35, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x36, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x37, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x38, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x39, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x3A, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x3B, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x3C, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x3D, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x3E, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x3F, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x40, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x41, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x42, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x43, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x44, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x45, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x46, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x47, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x48, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x49, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x4A, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x4B, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x4C, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x4D, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x4E, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x4F, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x50, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x51, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x52, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x53, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x54, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x55, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x56, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x57, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x58, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x59, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x5A, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x5B, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x5C, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x5D, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x5E, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x5F, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x60, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x61, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x62, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x63, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x64, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x65, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x66, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x67, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x68, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x69, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x6A, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x6B, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x6C, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x6D, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x6E, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x6F, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x70, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x71, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x72, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x73, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x74, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x75, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x76, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x77, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x78, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x79, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x7A, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x7B, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x7C, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x7D, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x7E, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0x7F, 0x0, 0x0], NonZeroU8::new_unchecked(1))),
            Some(([0xC2, 0xB0, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xB7, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xE2, 0x88, 0x99], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x88, 0x9A], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x96, 0x92], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0x80], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0x82], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0xBC], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0xA4], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0xAC], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0x9C], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0xB4], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0x90], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0x8C], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0x94], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x94, 0x98], NonZeroU8::new_unchecked(3))),
            Some(([0xCE, 0xB2, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xE2, 0x88, 0x9E], NonZeroU8::new_unchecked(3))),
            Some(([0xCF, 0x86, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xB1, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xBD, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xBC, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xE2, 0x89, 0x88], NonZeroU8::new_unchecked(3))),
            Some(([0xC2, 0xAB, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xBB, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBB, 0xB7], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xB8], NonZeroU8::new_unchecked(3))),
            None,
            None,
            Some(([0xEF, 0xBB, 0xBB], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xBC], NonZeroU8::new_unchecked(3))),
            None,
            Some(([0xC2, 0xA0, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xAD, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBA, 0x82], NonZeroU8::new_unchecked(3))),
            Some(([0xC2, 0xA3, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xA4, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBA, 0x84], NonZeroU8::new_unchecked(3))),
            None,
            None,
            Some(([0xEF, 0xBA, 0x8E], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x8F], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x95], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x99], NonZeroU8::new_unchecked(3))),
            Some(([0xD8, 0x8C, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBA, 0x9D], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xA1], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xA5], NonZeroU8::new_unchecked(3))),
            Some(([0xD9, 0xA0, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA1, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA2, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA3, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA4, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA5, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA6, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA7, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA8, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xD9, 0xA9, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBB, 0x91], NonZeroU8::new_unchecked(3))),
            Some(([0xD8, 0x9B, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBA, 0xB1], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xB5], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xB9], NonZeroU8::new_unchecked(3))),
            Some(([0xD8, 0x9F, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xA2, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBA, 0x80], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x81], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x83], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x85], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x8A], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x8B], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x8D], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x91], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x93], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x97], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x9B], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0x9F], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xA3], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xA7], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xA9], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xAB], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xAD], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xAF], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xB3], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xB7], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xBB], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xBF], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x81], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x85], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x8B], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x8F], NonZeroU8::new_unchecked(3))),
            Some(([0xC2, 0xA6, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC2, 0xAC, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC3, 0xB7, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xC3, 0x97, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBB, 0x89], NonZeroU8::new_unchecked(3))),
            Some(([0xD9, 0x80, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBB, 0x93], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x97], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x9B], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x9F], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xA3], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xA7], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xAB], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xAD], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xAF], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xB3], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBA, 0xBD], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x8C], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x8E], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x8D], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xA1], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xB9, 0xBD], NonZeroU8::new_unchecked(3))),
            Some(([0xD9, 0x91, 0x0], NonZeroU8::new_unchecked(2))),
            Some(([0xEF, 0xBB, 0xA5], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xA9], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xAC], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xB0], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xB2], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x90], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x95], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xB5], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xB6], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x9D], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0x99], NonZeroU8::new_unchecked(3))),
            Some(([0xEF, 0xBB, 0xB1], NonZeroU8::new_unchecked(3))),
            Some(([0xE2, 0x96, 0xA0], NonZeroU8::new_unchecked(3))),
            None,
        ]
    };
}
impl Encoder for CP864 {
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
                    0xB0 => 0x80,
                    0xB7 => 0x81,
                    0xB1 => 0x93,
                    0xBD => 0x94,
                    0xBC => 0x95,
                    0xAB => 0x97,
                    0xBB => 0x98,
                    0xA0 => 0xA0,
                    0xAD => 0xA1,
                    0xA3 => 0xA3,
                    0xA4 => 0xA4,
                    0xA2 => 0xC0,
                    0xA6 => 0xDB,
                    0xAC => 0xDC,
                    _ => return None,
                }
            }
            (0xC3, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0xB7 => 0xDD,
                    0x97 => 0xDE,
                    _ => return None,
                }
            }
            (0xCE, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0xB2 => 0x90,
                    _ => return None,
                }
            }
            (0xCF, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x86 => 0x92,
                    _ => return None,
                }
            }
            (0xD8, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x8C => 0xAC,
                    0x9B => 0xBB,
                    0x9F => 0xBF,
                    _ => return None,
                }
            }
            (0xD9, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0xAA => 0x25,
                    0xA0 => 0xB0,
                    0xA1 => 0xB1,
                    0xA2 => 0xB2,
                    0xA3 => 0xB3,
                    0xA4 => 0xB4,
                    0xA5 => 0xB5,
                    0xA6 => 0xB6,
                    0xA7 => 0xB7,
                    0xA8 => 0xB8,
                    0xA9 => 0xB9,
                    0x80 => 0xE0,
                    0x91 => 0xF1,
                    _ => return None,
                }
            }
            (0xE2, [_, b, c, ..]) => {
                *bytes = &bytes[3..];
                match b {
                    0x88 => match c {
                        0x99 => 0x82,
                        0x9A => 0x83,
                        0x9E => 0x91,
                        _ => return None,
                    },
                    0x89 => match c {
                        0x88 => 0x96,
                        _ => return None,
                    },
                    0x94 => match c {
                        0x80 => 0x85,
                        0x82 => 0x86,
                        0xBC => 0x87,
                        0xA4 => 0x88,
                        0xAC => 0x89,
                        0x9C => 0x8A,
                        0xB4 => 0x8B,
                        0x90 => 0x8C,
                        0x8C => 0x8D,
                        0x94 => 0x8E,
                        0x98 => 0x8F,
                        _ => return None,
                    },
                    0x96 => match c {
                        0x92 => 0x84,
                        0xA0 => 0xFE,
                        _ => return None,
                    },
                    _ => return None,
                }
            }
            (0xEF, [_, b, c, ..]) => {
                *bytes = &bytes[3..];
                match b {
                    0xB9 => match c {
                        0xBD => 0xF0,
                        _ => return None,
                    },
                    0xBA => match c {
                        0x82 => 0xA2,
                        0x84 => 0xA5,
                        0x8E => 0xA8,
                        0x8F => 0xA9,
                        0x95 => 0xAA,
                        0x99 => 0xAB,
                        0x9D => 0xAD,
                        0xA1 => 0xAE,
                        0xA5 => 0xAF,
                        0xB1 => 0xBC,
                        0xB5 => 0xBD,
                        0xB9 => 0xBE,
                        0x80 => 0xC1,
                        0x81 => 0xC2,
                        0x83 => 0xC3,
                        0x85 => 0xC4,
                        0x8B => 0xC6,
                        0x8D => 0xC7,
                        0x91 => 0xC8,
                        0x93 => 0xC9,
                        0x97 => 0xCA,
                        0x9B => 0xCB,
                        0x9F => 0xCC,
                        0xA3 => 0xCD,
                        0xA7 => 0xCE,
                        0xA9 => 0xCF,
                        0xAB => 0xD0,
                        0xAD => 0xD1,
                        0xAF => 0xD2,
                        0xB3 => 0xD3,
                        0xB7 => 0xD4,
                        0xBB => 0xD5,
                        0xBF => 0xD6,
                        0xBD => 0xEB,
                        _ => return None,
                    },
                    0xBB => match c {
                        0xB7 => 0x99,
                        0xB8 => 0x9A,
                        0xBB => 0x9D,
                        0xBC => 0x9E,
                        0x91 => 0xBA,
                        0x8A => 0xC5,
                        0x81 => 0xD7,
                        0x85 => 0xD8,
                        0x8B => 0xD9,
                        0x8F => 0xDA,
                        0x89 => 0xDF,
                        0x93 => 0xE1,
                        0x97 => 0xE2,
                        0x9B => 0xE3,
                        0x9F => 0xE4,
                        0xA3 => 0xE5,
                        0xA7 => 0xE6,
                        0xAB => 0xE7,
                        0xAD => 0xE8,
                        0xAF => 0xE9,
                        0xB3 => 0xEA,
                        0x8C => 0xEC,
                        0x8E => 0xED,
                        0x8D => 0xEE,
                        0xA1 => 0xEF,
                        0xA5 => 0xF2,
                        0xA9 => 0xF3,
                        0xAC => 0xF4,
                        0xB0 => 0xF5,
                        0xB2 => 0xF6,
                        0x90 => 0xF7,
                        0x95 => 0xF8,
                        0xB5 => 0xF9,
                        0xB6 => 0xFA,
                        0x9D => 0xFB,
                        0x99 => 0xFC,
                        0xB1 => 0xFD,
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
