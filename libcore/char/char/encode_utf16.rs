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

    // pub fn encode_utf16_raw(mut ch: u32, dst: &mut [u16]) -> Option<usize> {
    //     // Marked #[inline] to allow llvm optimizing it away
    //     if (ch & 0xFFFF) == ch && !dst.is_empty() {
    //         // The BMP falls through (assuming non-surrogate, as it should)
    //         dst[0] = ch as u16;
    //         Some(1)
    //     } else if dst.len() >= 2 {
    //         // Supplementary planes break into surrogates.
    //         ch -= 0x1_0000;
    //         dst[0] = 0xD800 | ((ch >> 10) as u16);
    //         dst[1] = 0xDC00 | ((ch as u16) & 0x3FF);
    //         Some(2)
    //     } else {
    //         None
    //     }
    // }

    #[test]
    fn encode_utf16_test1() {
	let beer_mug: char = '\u{1F37A}';
	let dst: &mut [u16] = &mut [0, 0];
	let len: Option<usize> = beer_mug.encode_utf16(dst);

	assert_eq!(len, Some::<usize>(2));
	assert_eq!(dst, &mut [0xd83c, 0xdf7a]);
    }
}
