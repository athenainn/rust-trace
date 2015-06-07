#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::char::EscapeUnicode;

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

    // #[derive(Clone)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub struct EscapeUnicode {
    //     c: char,
    //     state: EscapeUnicodeState
    // }

    // impl Iterator for EscapeUnicode {
    //     type Item = char;
    //
    //     fn next(&mut self) -> Option<char> {
    //         match self.state {
    //             EscapeUnicodeState::Backslash => {
    //                 self.state = EscapeUnicodeState::Type;
    //                 Some('\\')
    //             }
    //             EscapeUnicodeState::Type => {
    //                 self.state = EscapeUnicodeState::LeftBrace;
    //                 Some('u')
    //             }
    //             EscapeUnicodeState::LeftBrace => {
    //                 let mut n = 0;
    //                 while (self.c as u32) >> (4 * (n + 1)) != 0 {
    //                     n += 1;
    //                 }
    //                 self.state = EscapeUnicodeState::Value(n);
    //                 Some('{')
    //             }
    //             EscapeUnicodeState::Value(offset) => {
    //                 let v = match ((self.c as i32) >> (offset * 4)) & 0xf {
    //                     i @ 0 ... 9 => '0' as i32 + i,
    //                     i => 'a' as i32 + (i - 10)
    //                 };
    //                 if offset == 0 {
    //                     self.state = EscapeUnicodeState::RightBrace;
    //                 } else {
    //                     self.state = EscapeUnicodeState::Value(offset - 1);
    //                 }
    //                 Some(unsafe { transmute(v) })
    //             }
    //             EscapeUnicodeState::RightBrace => {
    //                 self.state = EscapeUnicodeState::Done;
    //                 Some('}')
    //             }
    //             EscapeUnicodeState::Done => None,
    //         }
    //     }
    // }

    #[test]
    fn next_test1() {
	let beer_mug: char = '\u{1F37A}';
	let mut escape_unicode: EscapeUnicode = beer_mug.escape_unicode();

	assert_eq!(escape_unicode.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_unicode.next(), Some::<char>('u'));	// type
	assert_eq!(escape_unicode.next(), Some::<char>('{'));	// left brace
	assert_eq!(escape_unicode.next(), Some::<char>('1'));	// value [0-9a-f]
	assert_eq!(escape_unicode.next(), Some::<char>('f'));	// value [0-9a-f]
	assert_eq!(escape_unicode.next(), Some::<char>('3'));	// value [0-9a-f]
	assert_eq!(escape_unicode.next(), Some::<char>('7'));	// value [0-9a-f]
	assert_eq!(escape_unicode.next(), Some::<char>('a'));	// value [0-9a-f]
	assert_eq!(escape_unicode.next(), Some::<char>('}'));	// right brace
	assert_eq!(escape_unicode.next(), None::<char>);	// done
    }
}
