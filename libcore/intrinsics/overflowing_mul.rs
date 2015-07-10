#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::overflowing_mul;

    // pub fn overflowing_mul<T>(a: T, b: T) -> T;

    macro_rules! overflowing_mul_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let value: $T = $value;
	    let other: $T = $other;
	    let result: $T = unsafe { overflowing_mul(value, other) };

	    assert_eq!(result, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn overflowing_mul_test1() {
	overflowing_mul_test!( u8, 0xff, 0x01, 0xff );
	overflowing_mul_test!( u8, 0xff, 0x02, 0xfe );

	overflowing_mul_test!( u16, 0xffff, 0x0001, 0xffff );
	overflowing_mul_test!( u16, 0xffff, 0x0002, 0xfffe );

	overflowing_mul_test!( u32, 0xffffffff, 0x00000001, 0xffffffff );
	overflowing_mul_test!( u32, 0xffffffff, 0x00000002, 0xfffffffe );

	overflowing_mul_test!( u64, 0xffffffffffffffff, 0x0000000000000001, 0xffffffffffffffff );
	overflowing_mul_test!( u64, 0xffffffffffffffff, 0x0000000000000002, 0xfffffffffffffffe );

	overflowing_mul_test!( i8, 0x7f, 0x01, 0x7f );
	overflowing_mul_test!( i8, 0x7f, 0x02, 0xfe );

	overflowing_mul_test!( i16, 0x7fff, 0x0001, 0x7fff );
	overflowing_mul_test!( i16, 0x7fff, 0x0002, 0xfffe );

	overflowing_mul_test!( i32, 0x7fffffff, 0x00000001, 0x7fffffff );
	overflowing_mul_test!( i32, 0x7fffffff, 0x00000002, 0xfffffffe );

	overflowing_mul_test!( i64, 0x7fffffffffffffff, 0x0000000000000001, 0x7fffffffffffffff );
	overflowing_mul_test!( i64, 0x7fffffffffffffff, 0x0000000000000002, 0xfffffffffffffffe );
    }
}
