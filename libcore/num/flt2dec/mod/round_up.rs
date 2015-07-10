#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::round_up;

    #[test]
    fn round_up_test1() {
	let value: &mut [u8] = &mut [
	    b'2', b'9', b'9',  b'7',  b'9',  b'2',  b'4',  b'5',  b'8'
	];
	let n: usize = 9;
	let result: Option<u8> = round_up(value, n);

	assert_eq!(value, &mut [
	    b'2', b'9', b'9',  b'7',  b'9',  b'2',  b'4',  b'5',  b'9'
	]);
	assert_eq!(result, None::<u8>);
    }

    #[test]
    fn round_up_test2() {
	let value: &mut [u8] = &mut [
	    b'2', b'9', b'9',  b'7',  b'9',  b'2',  b'4',  b'5',  b'8'
	];
	let n: usize = 5;
	let result: Option<u8> = round_up(value, n);

	assert_eq!(value, &mut [
	    b'2', b'9', b'9',  b'8',  b'0',  b'2',  b'4',  b'5',  b'8'
	]);
	assert_eq!(result, None::<u8>);
    }

    #[test]
    fn round_up_test3() {
	let value: &mut [u8] = &mut [
	    b'2', b'9', b'9',  b'7',  b'9',  b'2',  b'4',  b'5',  b'8'
	];
	let n: usize = 3;
	let result: Option<u8> = round_up(value, n);

	assert_eq!(value, &mut [
	    b'3', b'0', b'0',  b'7',  b'9',  b'2',  b'4',  b'5',  b'8'
	]);
	assert_eq!(result, None::<u8>);
    }

    #[test]
    fn round_up_test4() {
	let value: &mut [u8] = &mut [
	    b'9', b'9', b'9',  b'9',  b'9'
	];
	let n: usize = 5;
	let result: Option<u8> = round_up(value, n);

	assert_eq!(value, &mut [b'1', b'0', b'0',  b'0',  b'0']);
	assert_eq!(result, Some::<u8>(b'0'));
    }

    #[test]
    fn round_up_test5() {
	let value: &mut [u8] = &mut [];
	let n: usize = 0;
	let result: Option<u8> = round_up(value, n);

	assert_eq!(value, &mut []);
	assert_eq!(result, Some::<u8>(b'1'));
    }
}
