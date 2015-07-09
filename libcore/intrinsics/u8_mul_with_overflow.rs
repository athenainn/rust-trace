#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::u8_mul_with_overflow;

    // pub fn u8_mul_with_overflow(x: u8, y: u8) -> (u8, bool);

    #[test]
    fn u8_mul_with_overflow_test1() {
	let x: u8 = 0xff; // 255
	let y: u8 = 0x01; // 1

	let (result, is_overflow): (u8, bool) = unsafe {
	    u8_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0xff); // 255
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u8_mul_with_overflow_test2() {
	let x: u8 = 0xff; // 255
	let y: u8 = 0x02; // 2

	let (result, is_overflow): (u8, bool) = unsafe {
	    u8_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0xfe); // 254
	assert_eq!(is_overflow, true);
    }
}
