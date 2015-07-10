#![feature(core)]
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

    #[test]
    fn to_digit_test1() {
	let digits: &[char] = &[
	    '0', '1', '2', '3', '4',
	    '5', '6', '7', '8', '9'
	];
	let radix: u32 = 10;

	for num in 0..radix {
	    let digit: char = digits[num as usize];
	    let result: Option<u32> = digit.to_digit(radix);

	    assert_eq!(result, Some::<u32>(num));
	}
    }

    #[test]
    fn to_digit_test2() {
	let digits: &[char] = &[
	    '0', '1', '2', '3',
	    '4', '5', '6', '7',
	    '8', '9', 'a', 'b',
	    'c', 'd', 'e', 'f'
	];
	let radix: u32 = 16;

	for num in 0..radix {
	    let digit: char = digits[num as usize];
	    let result: Option<u32> = digit.to_digit(radix);

	    assert_eq!(result, Some::<u32>(num));
	}
    }

    #[test]
    fn to_digit_test3() {
	let digits: &[char] = &[
	    '0', '1', '2', '3',
	    '4', '5', '6', '7',
	    '8', '9', 'A', 'B',
	    'C', 'D', 'E', 'F'
	];
	let radix: u32 = 16;

	for num in 0..radix {
	    let digit: char = digits[num as usize];
	    let result: Option<u32> = digit.to_digit(radix);

	    assert_eq!(result, Some::<u32>(num));
	}
    }

    #[test]
    fn to_digit_test4() {
	let digits: &[char] = &[
	    '0', '1', '2', '3', '4',
	    '5', '6', '7', '8', '9',
	    'a', 'b', 'c', 'd', 'e',
	    'f', 'g', 'h', 'i', 'j',
	    'k', 'l', 'm', 'n', 'o',
	    'p', 'q', 'r', 's', 't',
	    'u', 'v', 'w', 'x', 'y',
	    'z'
	];
	let radix: u32 = 36;

	for num in 0..radix {
	    let digit: char = digits[num as usize];
	    let result: Option<u32> = digit.to_digit(radix);

	    assert_eq!(result, Some::<u32>(num));
	}
    }

    #[test]
    fn to_digit_test5() {
	let digits: &[char] = &[
	    '0', '1', '2', '3', '4',
	    '5', '6', '7', '8', '9',
	    'A', 'B', 'C', 'D', 'E',
	    'F', 'G', 'H', 'I', 'J',
	    'K', 'L', 'M', 'N', 'O',
	    'P', 'Q', 'R', 'S', 'T',
	    'U', 'V', 'W', 'X', 'Y',
	    'Z'
	];
	let radix: u32 = 36;

	for num in 0..radix {
	    let digit: char = digits[num as usize];
	    let result: Option<u32> = digit.to_digit(radix);

	    assert_eq!(result, Some::<u32>(num));
	}
    }

    #[test]
    #[should_panic]
    fn to_digit_test6() {
	let digit: char = '0';
	let radix: u32 = 37;
	let _: Option<u32> = digit.to_digit(radix); // panicked at 'to_digit: radix is too high (maximum 36)'
    }
}
