//! Code autogenerated from <https://unicode.org/Public/MAPPINGS/VENDORS/>
//! See binary codegen crate
use std::borrow::Cow;
use crate::{
    decoder::{self, complete::decode_helper, UTF8Entry, UTF8Len},
    encoder::Encoder, CodePage, DecodeError, EncodeError,
};
#[derive(Copy, Clone)]
pub struct CP855;
impl CP855 {
    /// Decode CP855 byte-encoding into UTF-8 string
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP855;
    ///
    /// assert_eq!(CP855.decode(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Cow<str> {
        decode_helper(&DECODE_TABLE, bytes)
    }
    /// Encode UTF-8 string into CP855 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP855;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP855.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP855.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }
    /// Encode UTF-8 string into CP855 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP855;
    ///
    /// assert_eq!(CP855.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}
impl CodePage for CP855 {
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        Ok((*self).decode(bytes))
    }
}
const DECODE_TABLE: decoder::complete::Table = [
    UTF8Entry {
        buf: [0x0, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x1, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x2, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x3, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x4, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x5, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x6, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x7, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x8, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x9, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0xA, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0xB, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0xC, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0xD, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0xE, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0xF, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x10, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x11, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x12, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x13, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x14, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x15, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x16, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x17, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x18, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x19, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x1A, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x1B, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x1C, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x1D, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x1E, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x1F, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x20, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x21, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x22, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x23, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x24, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x25, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x26, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x27, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x28, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x29, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x2A, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x2B, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x2C, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x2D, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x2E, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x2F, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x30, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x31, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x32, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x33, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x34, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x35, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x36, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x37, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x38, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x39, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x3A, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x3B, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x3C, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x3D, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x3E, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x3F, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x40, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x41, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x42, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x43, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x44, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x45, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x46, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x47, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x48, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x49, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x4A, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x4B, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x4C, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x4D, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x4E, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x4F, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x50, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x51, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x52, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x53, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x54, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x55, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x56, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x57, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x58, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x59, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x5A, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x5B, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x5C, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x5D, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x5E, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x5F, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x60, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x61, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x62, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x63, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x64, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x65, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x66, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x67, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x68, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x69, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x6A, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x6B, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x6C, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x6D, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x6E, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x6F, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x70, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x71, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x72, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x73, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x74, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x75, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x76, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x77, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x78, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x79, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x7A, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x7B, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x7C, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x7D, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x7E, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0x7F, 0x0, 0x0],
        len: UTF8Len::One,
    },
    UTF8Entry {
        buf: [0xD1, 0x92, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x82, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x93, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x83, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x91, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x81, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x94, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x84, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x95, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x85, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x96, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x86, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x97, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x87, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x98, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x88, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x99, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x89, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x9A, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x8A, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x9B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x8B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x9C, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x8C, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x9E, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x8E, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x9F, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x8F, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x8E, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xAE, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x8A, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xAA, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB0, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x90, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB1, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x91, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x86, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA6, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x94, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB5, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x95, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x84, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB3, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x93, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xAB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xBB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x96, 0x91],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x96, 0x92],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x96, 0x93],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0x82],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0xA4],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xD1, 0x85, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA5, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB8, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x98, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0xA3],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0x91],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0x97],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0x9D],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xD0, 0xB9, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x99, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0x90],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0x94],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0xB4],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0xAC],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0x9C],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0x80],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0xBC],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xD0, 0xBA, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x9A, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0x9A],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0x94],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0xA9],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0xA6],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0xA0],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0x90],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x95, 0xAC],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC2, 0xA4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xBB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x9B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xBC, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x9C, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xBD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x9D, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xBE, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x9E, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xBF, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0x98],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x94, 0x8C],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x96, 0x88],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x96, 0x84],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xD0, 0x9F, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x8F, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x96, 0x80],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xD0, 0xAF, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x80, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA0, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x81, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA1, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x82, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA2, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x83, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA3, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB6, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x96, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB2, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x92, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x8C, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xAC, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x84, 0x96],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC2, 0xAD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x8B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xAB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xB7, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0x97, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x88, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA8, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x8D, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xAD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x89, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA9, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD1, 0x87, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xD0, 0xA7, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xA7, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x96, 0xA0],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC2, 0xA0, 0x0],
        len: UTF8Len::Two,
    },
];
impl Encoder for CP855 {
    #[doc(hidden)]
    #[inline]
    fn encode_grapheme(&self, bytes: &mut &[u8]) -> Option<u8> {
        let (&a, rest) = bytes.split_first().unwrap();
        Some(
            match (a, &bytes) {
                (0x00..=0x7F, _) => {
                    *bytes = rest;
                    a
                }
                (0xC2, [_, b, ..]) => {
                    *bytes = &bytes[2..];
                    match b {
                        0xAB => 0xAE,
                        0xBB => 0xAF,
                        0xA4 => 0xCF,
                        0xAD => 0xF0,
                        0xA7 => 0xFD,
                        0xA0 => 0xFF,
                        _ => return None,
                    }
                }
                (0xD0, [_, b, ..]) => {
                    *bytes = &bytes[2..];
                    match b {
                        0x82 => 0x81,
                        0x83 => 0x83,
                        0x81 => 0x85,
                        0x84 => 0x87,
                        0x85 => 0x89,
                        0x86 => 0x8B,
                        0x87 => 0x8D,
                        0x88 => 0x8F,
                        0x89 => 0x91,
                        0x8A => 0x93,
                        0x8B => 0x95,
                        0x8C => 0x97,
                        0x8E => 0x99,
                        0x8F => 0x9B,
                        0xAE => 0x9D,
                        0xAA => 0x9F,
                        0xB0 => 0xA0,
                        0x90 => 0xA1,
                        0xB1 => 0xA2,
                        0x91 => 0xA3,
                        0xA6 => 0xA5,
                        0xB4 => 0xA6,
                        0x94 => 0xA7,
                        0xB5 => 0xA8,
                        0x95 => 0xA9,
                        0xA4 => 0xAB,
                        0xB3 => 0xAC,
                        0x93 => 0xAD,
                        0xA5 => 0xB6,
                        0xB8 => 0xB7,
                        0x98 => 0xB8,
                        0xB9 => 0xBD,
                        0x99 => 0xBE,
                        0xBA => 0xC6,
                        0x9A => 0xC7,
                        0xBB => 0xD0,
                        0x9B => 0xD1,
                        0xBC => 0xD2,
                        0x9C => 0xD3,
                        0xBD => 0xD4,
                        0x9D => 0xD5,
                        0xBE => 0xD6,
                        0x9E => 0xD7,
                        0xBF => 0xD8,
                        0x9F => 0xDD,
                        0xAF => 0xE0,
                        0xA0 => 0xE2,
                        0xA1 => 0xE4,
                        0xA2 => 0xE6,
                        0xA3 => 0xE8,
                        0xB6 => 0xE9,
                        0x96 => 0xEA,
                        0xB2 => 0xEB,
                        0x92 => 0xEC,
                        0xAC => 0xEE,
                        0xAB => 0xF2,
                        0xB7 => 0xF3,
                        0x97 => 0xF4,
                        0xA8 => 0xF6,
                        0xAD => 0xF8,
                        0xA9 => 0xFA,
                        0xA7 => 0xFC,
                        _ => return None,
                    }
                }
                (0xD1, [_, b, ..]) => {
                    *bytes = &bytes[2..];
                    match b {
                        0x92 => 0x80,
                        0x93 => 0x82,
                        0x91 => 0x84,
                        0x94 => 0x86,
                        0x95 => 0x88,
                        0x96 => 0x8A,
                        0x97 => 0x8C,
                        0x98 => 0x8E,
                        0x99 => 0x90,
                        0x9A => 0x92,
                        0x9B => 0x94,
                        0x9C => 0x96,
                        0x9E => 0x98,
                        0x9F => 0x9A,
                        0x8E => 0x9C,
                        0x8A => 0x9E,
                        0x86 => 0xA4,
                        0x84 => 0xAA,
                        0x85 => 0xB5,
                        0x8F => 0xDE,
                        0x80 => 0xE1,
                        0x81 => 0xE3,
                        0x82 => 0xE5,
                        0x83 => 0xE7,
                        0x8C => 0xED,
                        0x8B => 0xF1,
                        0x88 => 0xF5,
                        0x8D => 0xF7,
                        0x89 => 0xF9,
                        0x87 => 0xFB,
                        _ => return None,
                    }
                }
                (0xE2, [_, b, c, ..]) => {
                    *bytes = &bytes[3..];
                    match b {
                        0x84 => {
                            match c {
                                0x96 => 0xEF,
                                _ => return None,
                            }
                        }
                        0x94 => {
                            match c {
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
                            }
                        }
                        0x95 => {
                            match c {
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
                            }
                        }
                        0x96 => {
                            match c {
                                0x91 => 0xB0,
                                0x92 => 0xB1,
                                0x93 => 0xB2,
                                0x88 => 0xDB,
                                0x84 => 0xDC,
                                0x80 => 0xDF,
                                0xA0 => 0xFE,
                                _ => return None,
                            }
                        }
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
            },
        )
    }
}
