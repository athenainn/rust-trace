#![feature(core, unicode)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl CharExt for char {
    //     fn is_digit(self, radix: u32) -> bool {
    //         self.to_digit(radix).is_some()
    //     }
    //
    //     fn to_digit(self, radix: u32) -> Option<u32> {
    //         if radix > 36 {
    //             panic!("to_digit: radix is too high (maximum 36)");
    //         }
    //         let val = match self {
    //           '0' ... '9' => self as u32 - '0' as u32,
    //           'a' ... 'z' => self as u32 - 'a' as u32 + 10,
    //           'A' ... 'Z' => self as u32 - 'A' as u32 + 10,
    //           _ => return None,
    //         };
    //         if val < radix { Some(val) }
    //         else { None }
    //     }
    //
    //     fn escape_unicode(self) -> EscapeUnicode {
    //         EscapeUnicode { c: self, state: EscapeUnicodeState::Backslash }
    //     }
    //
    //     fn escape_default(self) -> EscapeDefault {
    //         let init_state = match self {
    //             '\t' => EscapeDefaultState::Backslash('t'),
    //             '\r' => EscapeDefaultState::Backslash('r'),
    //             '\n' => EscapeDefaultState::Backslash('n'),
    //             '\\' => EscapeDefaultState::Backslash('\\'),
    //             '\'' => EscapeDefaultState::Backslash('\''),
    //             '"'  => EscapeDefaultState::Backslash('"'),
    //             '\x20' ... '\x7e' => EscapeDefaultState::Char(self),
    //             _ => EscapeDefaultState::Unicode(self.escape_unicode())
    //         };
    //         EscapeDefault { state: init_state }
    //     }
    //
    //     #[inline]
    //     fn len_utf8(self) -> usize {
    //         let code = self as u32;
    //         if code < MAX_ONE_B {
    //             1
    //         } else if code < MAX_TWO_B {
    //             2
    //         } else if code < MAX_THREE_B {
    //             3
    //         } else {
    //             4
    //         }
    //     }
    //
    //     #[inline]
    //     fn len_utf16(self) -> usize {
    //         let ch = self as u32;
    //         if (ch & 0xFFFF) == ch { 1 } else { 2 }
    //     }
    //
    //     #[inline]
    //     fn encode_utf8(self, dst: &mut [u8]) -> Option<usize> {
    //         encode_utf8_raw(self as u32, dst)
    //     }
    //
    //     #[inline]
    //     fn encode_utf16(self, dst: &mut [u16]) -> Option<usize> {
    //         encode_utf16_raw(self as u32, dst)
    //     }
    // }

    // const TAG_CONT: u8    = 0b1000_0000;
    // const TAG_TWO_B: u8   = 0b1100_0000;
    // const TAG_THREE_B: u8 = 0b1110_0000;
    // const TAG_FOUR_B: u8  = 0b1111_0000;
    // const MAX_ONE_B: u32   =     0x80;
    // const MAX_TWO_B: u32   =    0x800;
    // const MAX_THREE_B: u32 =  0x10000;

    // pub fn encode_utf8_raw(code: u32, dst: &mut [u8]) -> Option<usize> {
    //     // Marked #[inline] to allow llvm optimizing it away
    //     if code < MAX_ONE_B && !dst.is_empty() {
    //         dst[0] = code as u8;
    //         Some(1)
    //     } else if code < MAX_TWO_B && dst.len() >= 2 {
    //         dst[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
    //         dst[1] = (code & 0x3F) as u8 | TAG_CONT;
    //         Some(2)
    //     } else if code < MAX_THREE_B && dst.len() >= 3  {
    //         dst[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
    //         dst[1] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
    //         dst[2] = (code & 0x3F) as u8 | TAG_CONT;
    //         Some(3)
    //     } else if dst.len() >= 4 {
    //         dst[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
    //         dst[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
    //         dst[2] = (code >>  6 & 0x3F) as u8 | TAG_CONT;
    //         dst[3] = (code & 0x3F) as u8 | TAG_CONT;
    //         Some(4)
    //     } else {
    //         None
    //     }
    // }

    #[test]
    fn encode_utf8_test1() {
	let beer_mug: char = '\u{1F37A}';
	let dst: &mut [u8] = &mut [0, 0, 0, 0];
	let len: Option<usize> = beer_mug.encode_utf8(dst);

	assert_eq!(len, Some::<usize>(4));
	assert_eq!(dst, &mut [0xf0, 0x9f, 0x8d, 0xba]);
    }
}
