// Code autogenerated from https://unicode.org/Public/MAPPINGS/VENDORS/
// See binary codegen crate
use crate::internal::decoder_complete;
use crate::internal::decoder_complete::decode_helper;
use crate::internal::{Encoder, UTF8Entry, UTF8Len};
use crate::{CodePage, DecodeError, EncodeError};
use std::borrow::Cow;

#[derive(Copy, Clone)]
pub struct CP1250;

impl CP1250 {
    /// Decode CP1250 byte-encoding into UTF-8 string
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1250;
    ///
    /// assert_eq!(CP1250.decode(&[116, 101, 120, 116]), "text");
    /// ```
    #[inline(always)]
    pub fn decode(self, bytes: &[u8]) -> Cow<str> {
        decode_helper(&DECODE_TABLE, bytes)
    }

    /// Encode UTF-8 string into CP1250 byte-encoding
    ///
    /// Undefined characters will result in [`EncodeError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1250;
    /// use yore::EncodeError;
    ///
    /// assert_eq!(CP1250.encode("text").unwrap(), vec![116, 101, 120, 116]);
    /// assert!(matches!(CP1250.encode("text 🦀"), EncodeError));
    /// ```
    #[inline(always)]
    pub fn encode(self, s: &str) -> Result<Cow<[u8]>, EncodeError> {
        self.encode_helper(s, None)
    }

    /// Encode UTF-8 string into CP1250 byte-encoding
    ///
    /// Undefined characters will be replaced with byte `fallback`
    ///
    /// # Examples
    ///
    /// ```
    /// use yore::code_pages::CP1250;
    ///
    /// assert_eq!(CP1250.encode_lossy("text 🦀", 168), vec![116, 101, 120, 116, 32, 168]);
    /// ```
    #[inline(always)]
    pub fn encode_lossy(self, s: &str, fallback: u8) -> Cow<[u8]> {
        self.encode_helper(s, Some(fallback)).unwrap()
    }
}
impl CodePage for CP1250 {
    #[inline(always)]
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<Cow<'a, str>, DecodeError> {
        Ok((*self).decode(bytes))
    }
}

const DECODE_TABLE: decoder_complete::Table = [
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
        buf: [0xE2, 0x82, 0xAC],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC2, 0x81, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x9A],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC2, 0x83, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x9E],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0xA6],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0xA0],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0xA1],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC2, 0x88, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0xB0],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC5, 0xA0, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0xB9],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC5, 0x9A, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xA4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xBD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xB9, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0x90, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x98],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x99],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x9C],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x9D],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0xA2],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x93],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0x94],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC2, 0x98, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x84, 0xA2],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC5, 0xA1, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xE2, 0x80, 0xBA],
        len: UTF8Len::Three,
    },
    UTF8Entry {
        buf: [0xC5, 0x9B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xA5, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xBE, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xBA, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xA0, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xCB, 0x87, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xCB, 0x98, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x81, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xA4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x84, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xA6, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xA7, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xA8, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xA9, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x9E, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xAB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xAC, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xAD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xAE, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xBB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xB0, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xB1, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xCB, 0x9B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x82, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xB4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xB5, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xB6, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xB7, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xB8, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x85, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x9F, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC2, 0xBB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0xBD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xCB, 0x9D, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0xBE, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xBC, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x94, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x81, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x82, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x82, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x84, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0xB9, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x86, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x87, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x8C, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x89, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x98, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x8B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x9A, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x8D, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x8E, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x8E, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x90, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x83, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x87, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x93, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x94, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x90, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x96, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x97, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x98, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xAE, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x9A, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xB0, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x9C, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x9D, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xA2, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0x9F, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x95, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xA1, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xA2, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x83, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xA4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0xBA, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x87, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xA7, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x8D, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xA9, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x99, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xAB, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x9B, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xAD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xAE, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x8F, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC4, 0x91, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x84, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x88, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xB3, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xB4, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x91, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xB6, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xB7, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0x99, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xAF, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xBA, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xB1, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xBC, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC3, 0xBD, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xC5, 0xA3, 0x0],
        len: UTF8Len::Two,
    },
    UTF8Entry {
        buf: [0xCB, 0x99, 0x0],
        len: UTF8Len::Two,
    },
];
impl Encoder for CP1250 {
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
                    0x83 => 0x83,
                    0x88 => 0x88,
                    0x90 => 0x90,
                    0x98 => 0x98,
                    0xA0 => 0xA0,
                    0xA4 => 0xA4,
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
                    0xB4 => 0xB4,
                    0xB5 => 0xB5,
                    0xB6 => 0xB6,
                    0xB7 => 0xB7,
                    0xB8 => 0xB8,
                    0xBB => 0xBB,
                    _ => return None,
                }
            }
            (0xC3, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x81 => 0xC1,
                    0x82 => 0xC2,
                    0x84 => 0xC4,
                    0x87 => 0xC7,
                    0x89 => 0xC9,
                    0x8B => 0xCB,
                    0x8D => 0xCD,
                    0x8E => 0xCE,
                    0x93 => 0xD3,
                    0x94 => 0xD4,
                    0x96 => 0xD6,
                    0x97 => 0xD7,
                    0x9A => 0xDA,
                    0x9C => 0xDC,
                    0x9D => 0xDD,
                    0x9F => 0xDF,
                    0xA1 => 0xE1,
                    0xA2 => 0xE2,
                    0xA4 => 0xE4,
                    0xA7 => 0xE7,
                    0xA9 => 0xE9,
                    0xAB => 0xEB,
                    0xAD => 0xED,
                    0xAE => 0xEE,
                    0xB3 => 0xF3,
                    0xB4 => 0xF4,
                    0xB6 => 0xF6,
                    0xB7 => 0xF7,
                    0xBA => 0xFA,
                    0xBC => 0xFC,
                    0xBD => 0xFD,
                    _ => return None,
                }
            }
            (0xC4, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x84 => 0xA5,
                    0x85 => 0xB9,
                    0xBD => 0xBC,
                    0xBE => 0xBE,
                    0x82 => 0xC3,
                    0xB9 => 0xC5,
                    0x86 => 0xC6,
                    0x8C => 0xC8,
                    0x98 => 0xCA,
                    0x9A => 0xCC,
                    0x8E => 0xCF,
                    0x90 => 0xD0,
                    0x83 => 0xE3,
                    0xBA => 0xE5,
                    0x87 => 0xE6,
                    0x8D => 0xE8,
                    0x99 => 0xEA,
                    0x9B => 0xEC,
                    0x8F => 0xEF,
                    0x91 => 0xF0,
                    _ => return None,
                }
            }
            (0xC5, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0xA0 => 0x8A,
                    0x9A => 0x8C,
                    0xA4 => 0x8D,
                    0xBD => 0x8E,
                    0xB9 => 0x8F,
                    0xA1 => 0x9A,
                    0x9B => 0x9C,
                    0xA5 => 0x9D,
                    0xBE => 0x9E,
                    0xBA => 0x9F,
                    0x81 => 0xA3,
                    0x9E => 0xAA,
                    0xBB => 0xAF,
                    0x82 => 0xB3,
                    0x9F => 0xBA,
                    0xBC => 0xBF,
                    0x94 => 0xC0,
                    0x83 => 0xD1,
                    0x87 => 0xD2,
                    0x90 => 0xD5,
                    0x98 => 0xD8,
                    0xAE => 0xD9,
                    0xB0 => 0xDB,
                    0xA2 => 0xDE,
                    0x95 => 0xE0,
                    0x84 => 0xF1,
                    0x88 => 0xF2,
                    0x91 => 0xF5,
                    0x99 => 0xF8,
                    0xAF => 0xF9,
                    0xB1 => 0xFB,
                    0xA3 => 0xFE,
                    _ => return None,
                }
            }
            (0xCB, [_, b, ..]) => {
                *bytes = &bytes[2..];
                match b {
                    0x87 => 0xA1,
                    0x98 => 0xA2,
                    0x9B => 0xB2,
                    0x9D => 0xBD,
                    0x99 => 0xFF,
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
