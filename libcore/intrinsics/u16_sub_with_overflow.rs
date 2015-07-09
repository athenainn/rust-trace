#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::u16_sub_with_overflow;

    // pub fn u16_sub_with_overflow(x: u16, y: u16) -> (u16, bool);

    #[test]
    fn u16_sub_with_overflow_test1() {
	let x: u16 = 0xff00; // 65280
	let y: u16 = 0x00ff; // 255

	let (result, is_overflow): (u16, bool) = unsafe {
	    u16_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0xfe01); // 65025
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u16_sub_with_overflow_test2() {
	let x: u16 = 0x0000; // 0
	let y: u16 = 0x0001; // 1

	let (result, is_overflow): (u16, bool) = unsafe {
	    u16_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0xffff); // 32767
	assert_eq!(is_overflow, true);
    }
}
