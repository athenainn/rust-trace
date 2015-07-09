#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::u8_sub_with_overflow;

    // pub fn u8_sub_with_overflow(x: u8, y: u8) -> (u8, bool);

    #[test]
    fn u8_sub_with_overflow_test1() {
	let x: u8 = 0xf0; // 240
	let y: u8 = 0x0f; // 15

	let (result, is_overflow): (u8, bool) = unsafe {
	    u8_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0xe1); // 225
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u8_sub_with_overflow_test2() {
	let x: u8 = 0x0; // 0
	let y: u8 = 0x1; // 1

	let (result, is_overflow): (u8, bool) = unsafe {
	    u8_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0xff); // 255
	assert_eq!(is_overflow, true);
    }
}
