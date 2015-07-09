#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::u16_mul_with_overflow;

    // pub fn u16_mul_with_overflow(x: u16, y: u16) -> (u16, bool);

    #[test]
    fn u16_mul_with_overflow_test1() {
	let x: u16 = 0xffff; // 65535
	let y: u16 = 0x0001; // 1

	let (result, is_overflow): (u16, bool) = unsafe {
	    u16_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0xffff); // 65535
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u16_mul_with_overflow_test2() {
	let x: u16 = 0xffff; // 65535
	let y: u16 = 0x0002; // 2

	let (result, is_overflow): (u16, bool) = unsafe {
	    u16_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0xfffe); // 65534
	assert_eq!(is_overflow, true);
    }
}
