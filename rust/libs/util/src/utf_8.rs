use crate::padded_string::*;
use crate::qljs_assert;
use crate::qljs_const_assert;

// Returns the number of bytes written.
//
// See: https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf
pub fn encode_utf_8<'a>(code_point: u32, out: &'a mut [u8]) -> usize {
    let byte = |x: u32| {
        qljs_assert!(x <= 0x100);
        x as u8
    };
    let continuation = 0b1000_0000;
    if code_point >= 0x10000 {
        out[0] = byte(0b1111_0000 | (code_point >> 18));
        out[1] = byte(continuation | ((code_point >> 12) & 0b0011_1111));
        out[2] = byte(continuation | ((code_point >> 6) & 0b0011_1111));
        out[3] = byte(continuation | ((code_point >> 0) & 0b0011_1111));
        4
    } else if code_point >= 0x0800 {
        out[0] = byte(0b1110_0000 | (code_point >> 12));
        out[1] = byte(continuation | ((code_point >> 6) & 0b0011_1111));
        out[2] = byte(continuation | ((code_point >> 0) & 0b0011_1111));
        3
    } else if code_point >= 0x80 {
        out[0] = byte(0b1100_0000 | (code_point >> 6));
        out[1] = byte(continuation | ((code_point >> 0) & 0b0011_1111));
        2
    } else {
        out[0] = byte(code_point);
        1
    }
}

pub struct DecodeUTF8Result {
    pub size: PaddedStringSizeType,
    pub code_point: Option<char>,
}

impl DecodeUTF8Result {
    pub fn is_ok(&self) -> bool {
        self.code_point.is_some()
    }
}

// See: https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf
pub fn decode_utf_8<'a>(input: PaddedStringView<'a>) -> DecodeUTF8Result {
    fn make_char(data: u32) -> char {
        unsafe { std::char::from_u32_unchecked(data) }
    }
    fn is_continuation_byte(byte: u8) -> bool {
        (byte & 0b1100_0000) == 0b1000_0000
    }
    let input_slice: &[u8] = input.slice_with_padding();
    let c = |index: usize| unsafe { *input_slice.get_unchecked(index) };
    if input.len() == 0 {
        DecodeUTF8Result {
            size: 0,
            code_point: None,
        }
    } else if c(0) <= 0x7f {
        // 1-byte sequence (0x00..0x7f, i.e. ASCII).
        DecodeUTF8Result {
            size: 1,
            code_point: Some(make_char(c(0) as u32)),
        }
    } else if (c(0) & 0b1110_0000) == 0b1100_0000 {
        // 2-byte sequence (0xc0..0xdf).
        qljs_const_assert!(PADDED_STRING_PADDING_SIZE >= 1);
        let byte_0_ok = c(0) >= 0xc2;
        let byte_1_ok = is_continuation_byte(c(1));
        if byte_0_ok && byte_1_ok {
            DecodeUTF8Result {
                size: 2,
                code_point: Some(make_char(
                    (((c(0) & 0b0001_1111) as u32) << 6) | ((c(1) & 0b0011_1111) as u32),
                )),
            }
        } else {
            DecodeUTF8Result {
                size: 1,
                code_point: None,
            }
        }
    } else if (c(0) & 0b1111_0000) == 0b1110_0000 {
        // 3-byte sequence (0xe0..0xef).
        qljs_const_assert!(PADDED_STRING_PADDING_SIZE >= 2);
        let byte_1_ok = if c(0) == 0xe0 {
            0xa0 <= c(1) && c(1) <= 0xbf
        } else if c(0) == 0xed {
            0x80 <= c(1) && c(1) <= 0x9f
        } else {
            is_continuation_byte(c(1))
        };
        let byte_2_ok = is_continuation_byte(c(2));
        if byte_1_ok && byte_2_ok {
            DecodeUTF8Result {
                size: 3,
                code_point: Some(make_char(
                    (((c(0) & 0b0000_1111) as u32) << 12)
                        | (((c(1) & 0b0011_1111) as u32) << 6)
                        | ((c(2) & 0b0011_1111) as u32),
                )),
            }
        } else {
            DecodeUTF8Result {
                size: if byte_1_ok { 2 } else { 1 },
                code_point: None,
            }
        }
    } else if (c(0) & 0b1111_1000) == 0b1111_0000 {
        // 4-byte sequence (0xf0..0xf7).
        qljs_const_assert!(PADDED_STRING_PADDING_SIZE >= 3);
        let byte_0_ok = c(0) <= 0xf4;
        let byte_1_ok = if c(0) == 0xf0 {
            0x90 <= c(1) && c(1) <= 0xbf
        } else if c(0) == 0xf4 {
            0x80 <= c(1) && c(1) <= 0x8f
        } else {
            is_continuation_byte(c(1))
        };
        let byte_2_ok = is_continuation_byte(c(2));
        let byte_3_ok = is_continuation_byte(c(3));
        if byte_0_ok && byte_1_ok && byte_2_ok && byte_3_ok {
            DecodeUTF8Result {
                size: 4,
                code_point: Some(make_char(
                    (((c(0) & 0b0000_0111) as u32) << 18)
                        | (((c(1) & 0b0011_1111) as u32) << 12)
                        | (((c(2) & 0b0011_1111) as u32) << 6)
                        | ((c(3) & 0b0011_1111) as u32),
                )),
            }
        } else {
            DecodeUTF8Result {
                size: if byte_0_ok && byte_1_ok {
                    if byte_2_ok {
                        3
                    } else {
                        2
                    }
                } else {
                    1
                },
                code_point: None,
            }
        }
    } else {
        // Continuation byte (0x80..0xbf), or 5-byte or longer sequence
        // (0xf8..0xff).
        DecodeUTF8Result {
            size: 1,
            code_point: None,
        }
    }
}

pub fn count_lsp_characters_in_utf_8(utf_8: PaddedStringView, offset: i32) -> isize {
    let mut c: PaddedStringSizeType = 0;
    let stop: PaddedStringSizeType = offset;
    let mut count: isize = 0;
    while c < stop {
        let result: DecodeUTF8Result = decode_utf_8(utf_8.substr(c));
        if let Some(code_point) = result.code_point {
            if c + result.size > stop {
                break;
            }
            c += result.size;
            if (code_point as u32) >= 0x10000 {
                count += 2;
            } else {
                count += 1;
            }
        } else {
            c += 1;
            count += 1;
        }
    }
    count
}

pub fn count_utf_8_characters(utf_8: PaddedStringView, offset: usize) -> usize {
    let mut c: usize = 0;
    let stop: usize = offset;
    let mut count: usize = 0;

    while c < stop {
        let result = decode_utf_8(utf_8.substr(c as PaddedStringSizeType));
        if !result.is_ok() {
            c += 1;
            count += 1;
            continue;
        }

        if c + (result.size as usize) > stop {
            break;
        }
        c += result.size as usize;
        count += 1;
    }

    count
}
