#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::i64_mul_with_overflow;

    // pub fn i64_mul_with_overflow(x: i64, y: i64) -> (i64, bool);

    #[test]
    fn i64_mul_with_overflow_test1() {
	let x: i64 = 0x7fffffffffffffff; // 9223372036854775807
	let y: i64 = 0x0000000000000001; // 1

	let (result, is_overflow): (i64, bool) = unsafe {
	    i64_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0x7fffffffffffffff); // 9223372036854775807
	assert_eq!(is_overflow, false);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i64_mul_with_overflow_test2() {
	let x: i64 = 0x7fffffffffffffff; // 9223372036854775807
	let y: i64 = 0x0000000000000002; // 2

	let (result, is_overflow): (i64, bool) = unsafe {
	    i64_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0xfffffffffffffffe); // -2
	assert_eq!(is_overflow, true);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i64_mul_with_overflow_test3() {
	let x: i64 = 0x8000000000000000; // -9223372036854775808
	let y: i64 = 0x0000000000000002; // 2

	let (result, is_overflow): (i64, bool) = unsafe {
	    i64_mul_with_overflow(x, y)
	};

	assert_eq!(result, 0x0); // 0
	assert_eq!(is_overflow, true);
    }
}
