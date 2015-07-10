#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::char::from_digit;

    // pub fn from_digit(num: u32, radix: u32) -> Option<char> {
    //     if radix > 36 {
    //         panic!("from_digit: radix is too high (maximum 36)");
    //     }
    //     if num < radix {
    //         unsafe {
    //             if num < 10 {
    //                 Some(transmute('0' as u32 + num))
    //             } else {
    //                 Some(transmute('a' as u32 + num - 10))
    //             }
    //         }
    //     } else {
    //         None
    //     }
    // }

    #[test]
    fn from_digit_test1() {
	let digits: &[char] = &[
	    '0', '1', '2', '3', '4',
	    '5', '6', '7', '8', '9'
	];
	let radix: u32 = 10;

	for num in 0..radix {
	    let result: Option<char> = from_digit(num, radix);
	    match result {
		Some(v) => assert_eq!(v, digits[num as usize]),
		None => assert!(false)
	    }
	}
    }

    #[test]
    fn from_digit_test2() {
	let digits: &[char] = &[
	    '0', '1', '2', '3',
	    '4', '5', '6', '7',
	    '8', '9', 'a', 'b',
	    'c', 'd', 'e', 'f'
	];
	let radix: u32 = 16;

	for num in 0..radix {
	    let result: Option<char> = from_digit(num, radix);
	    match result {
		Some(v) => assert_eq!(v, digits[num as usize]),
		None => assert!(false)
	    }
	}
    }

    #[test]
    fn from_digit_test3() {
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
	    let result: Option<char> = from_digit(num, radix);
	    match result {
		Some(v) => assert_eq!(v, digits[num as usize]),
		None => assert!(false)
	    }
	}
    }

    #[test]
    #[should_panic]
    fn from_digit_test4() {
	let num: u32 = 0;
	let radix: u32 = 37;
	let _: Option<char> = from_digit(num, radix); // panicked at 'from_digit: radix is too high (maximum 36)'
    }
}
