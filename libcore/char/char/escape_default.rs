#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::char::EscapeDefault;

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
    // pub struct EscapeDefault {
    //     state: EscapeDefaultState
    // }

    #[test]
    fn escape_default_test1() {
	let escape: char = '\t'; // '\t' == 0x9
	let mut escape_default: EscapeDefault = escape.escape_default();

	assert_eq!(escape_default.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_default.next(), Some::<char>('t'));	// char
	assert_eq!(escape_default.next(), None::<char>);	// done
    }

    #[test]
    fn escape_default_test2() {
	let escape: char = '\r'; // '\r' == 0xd
	let mut escape_default: EscapeDefault = escape.escape_default();

	assert_eq!(escape_default.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_default.next(), Some::<char>('r'));	// char
	assert_eq!(escape_default.next(), None::<char>);	// done
    }

    #[test]
    fn escape_default_test3() {
	let escape: char = '\n'; // '\n' == 0xa
	let mut escape_default: EscapeDefault = escape.escape_default();

	assert_eq!(escape_default.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_default.next(), Some::<char>('n'));	// char
	assert_eq!(escape_default.next(), None::<char>);	// done
    }

    #[test]
    fn escape_default_test4() {
	let escape: char = '\\'; // '\\' == 0x5c
	let mut escape_default: EscapeDefault = escape.escape_default();

	assert_eq!(escape_default.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_default.next(), Some::<char>('\\'));	// char
	assert_eq!(escape_default.next(), None::<char>);	// done
    }

    #[test]
    fn escape_default_test5() {
	let escape: char = '\''; // '\'' == 0x27
	let mut escape_default: EscapeDefault = escape.escape_default();

	assert_eq!(escape_default.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_default.next(), Some::<char>('\''));	// char
	assert_eq!(escape_default.next(), None::<char>);	// done
    }

    #[test]
    fn escape_default_test6() {
	let escape: char = '"'; // '"' == 0x22
	let mut escape_default: EscapeDefault = escape.escape_default();

	assert_eq!(escape_default.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_default.next(), Some::<char>('"'));	// char
	assert_eq!(escape_default.next(), None::<char>);	// done
    }

    #[test]
    fn escape_default_test7() {
	let escapes: &[char] = &[
	    '\x20', '\x21', /*\r*/  '\x23', '\x24', '\x25', '\x26', /*\'*/
	    '\x28', '\x29', '\x2a', '\x2b', '\x2c', '\x2d', '\x2e', '\x2f',
	    '\x30', '\x31', '\x32', '\x33', '\x34', '\x35', '\x36', '\x37',
	    '\x38', '\x39', '\x3a', '\x3b', '\x3c', '\x3d', '\x3e', '\x3f',
	    '\x40', '\x41', '\x42', '\x43', '\x44', '\x45', '\x46', '\x47',
	    '\x48', '\x49', '\x4a', '\x4b', '\x4c', '\x4d', '\x4e', '\x4f',
	    '\x50', '\x51', '\x52', '\x53', '\x54', '\x55', '\x56', '\x57',
	    '\x58', '\x59', '\x5a', '\x5b', /*\\*/  '\x5d', '\x5e', '\x5f',
	    '\x60', '\x61', '\x62', '\x63', '\x64', '\x65', '\x66', '\x67',
	    '\x68', '\x69', '\x6a', '\x6b', '\x6c', '\x6d', '\x6e', '\x6f',
	    '\x70', '\x71', '\x72', '\x73', '\x74', '\x75', '\x76', '\x77',
	    '\x78', '\x79', '\x7a', '\x7b', '\x7c', '\x7d', '\x7e'
	];

	for c in escapes {
	    let mut escape_default: EscapeDefault = c.escape_default();

	    assert_eq!(escape_default.next(), Some::<char>(*c));
	    assert_eq!(escape_default.next(), None::<char>);
	}
    }

    #[test]
    fn escape_default_test8() {
	let beer_mug: char = '\u{1F37A}';
	let mut escape_default: EscapeDefault = beer_mug.escape_default();

	assert_eq!(escape_default.next(), Some::<char>('\\'));	// back slash
	assert_eq!(escape_default.next(), Some::<char>('u'));	// type
	assert_eq!(escape_default.next(), Some::<char>('{'));	// left brace
	assert_eq!(escape_default.next(), Some::<char>('1'));	// value [0-9a-f]
	assert_eq!(escape_default.next(), Some::<char>('f'));	// value [0-9a-f]
	assert_eq!(escape_default.next(), Some::<char>('3'));	// value [0-9a-f]
	assert_eq!(escape_default.next(), Some::<char>('7'));	// value [0-9a-f]
	assert_eq!(escape_default.next(), Some::<char>('a'));	// value [0-9a-f]
	assert_eq!(escape_default.next(), Some::<char>('}'));	// right brace
	assert_eq!(escape_default.next(), None::<char>);	// done
    }
}
