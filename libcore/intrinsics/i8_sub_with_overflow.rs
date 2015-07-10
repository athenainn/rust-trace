#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::i8_sub_with_overflow;

    // pub fn i8_sub_with_overflow(x: i8, y: i8) -> (i8, bool);

    #[test]
    fn i8_sub_with_overflow_test1() {
	let x: i8 = 0x70; // 112
	let y: i8 = 0x0f; // 15

	let (result, is_overflow): (i8, bool) = unsafe {
	    i8_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0x61); // 97
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i8_sub_with_overflow_test2() {
	let x: i8 = 0x7f; // 127
	let y: i8 = 0xff; // -1

	let (result, is_overflow): (i8, bool) = unsafe {
	    i8_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0x80); // -1
	assert_eq!(is_overflow, true);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i8_sub_with_overflow_test3() {
	let x: i8 = 0x80; // -128
	let y: i8 = 0x01; // 1

	let (result, is_overflow): (i8, bool) = unsafe {
	    i8_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0x7f); // 127
	assert_eq!(is_overflow, true);
    }
}
