#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::u32_sub_with_overflow;

    // pub fn u32_sub_with_overflow(x: u32, y: u32) -> (u32, bool);

    #[test]
    fn u32_sub_with_overflow_test1() {
	let x: u32 = 0xffff0000; // 4294901760
	let y: u32 = 0x0000ffff; // 65535

	let (result, is_overflow): (u32, bool) = unsafe {
	    u32_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0xfffe0001); // 4294836225
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u32_sub_with_overflow_test2() {
	let x: u32 = 0x00000000; // 0
	let y: u32 = 0x00000001; // 1

	let (result, is_overflow): (u32, bool) = unsafe {
	    u32_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0xffffffff); // 4294967295
	assert_eq!(is_overflow, true);
    }
}
