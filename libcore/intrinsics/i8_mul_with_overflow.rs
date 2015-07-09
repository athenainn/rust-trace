#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::i8_mul_with_overflow;

    // pub fn i8_mul_with_overflow(x: i8, y: i8) -> (i8, bool);

    #[test]
    fn i8_mul_with_overflow_test1() {
	let x: i8 = 0x7f; // 127
	let y: i8 = 0x01; // 1

	let (result, is_overflow): (i8, bool) = unsafe {
	    i8_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0x7f); // 127
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i8_mul_with_overflow_test2() {
	let x: i8 = 0x7f; // 127
	let y: i8 = 0x02; // 2

	let (result, is_overflow): (i8, bool) = unsafe {
	    i8_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0xfe); // -2
	assert_eq!(is_overflow, true);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i8_mul_with_overflow_test3() {
	let x: i8 = 0x80; // -128
	let y: i8 = 0x02; // 2

	let (result, is_overflow): (i8, bool) = unsafe {
	    i8_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0x0); // 0
	assert_eq!(is_overflow, true);
    }
}
