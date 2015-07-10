#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::i32_sub_with_overflow;

    // pub fn i32_sub_with_overflow(x: i32, y: i32) -> (i32, bool);

    #[test]
    fn i32_sub_with_overflow_test1() {
	let x: i32 = 0x7fff0000; // 2147418112
	let y: i32 = 0x0000ffff; // 65535

	let (result, is_overflow): (i32, bool) = unsafe {
	    i32_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0x7ffe0001); // 214352577
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i32_sub_with_overflow_test2() {
	let x: i32 = 0x7fffffff; // 2147483647
	let y: i32 = 0xffffffff; // -1

	let (result, is_overflow): (i32, bool) = unsafe {
	    i32_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0x80000000); // -2147483648
	assert_eq!(is_overflow, true);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i32_sub_with_overflow_test3() {
	let x: i32 = 0x80000000; // -2147483648
	let y: i32 = 0x00000001; // 1

	let (result, is_overflow): (i32, bool) = unsafe {
	    i32_sub_with_overflow(x, y)
	};

	assert_eq!(result, 0x7fffffff); // 2147483647
	assert_eq!(is_overflow, true);
    }
}
