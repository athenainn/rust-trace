#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::overflowing_sub;

    // pub fn overflowing_sub<T>(a: T, b: T) -> T;

    macro_rules! overflowing_sub_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let value: $T = $value;
	    let other: $T = $other;
	    let result: $T = unsafe { overflowing_sub(value, other) };

	    assert_eq!(result, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn overflowing_sub_test1() {
	overflowing_sub_test!( u8, 0xf0, 0x0f, 0xe1 );
	overflowing_sub_test!( u8, 0x00, 0x01, 0xff );

	overflowing_sub_test!( u16, 0xff00, 0x00ff, 0xfe01 );
	overflowing_sub_test!( u16, 0x0000, 0x0001, 0xffff );

	overflowing_sub_test!( u32, 0xffff0000, 0x0000ffff, 0xfffe0001 );
	overflowing_sub_test!( u32, 0x00000000, 0x00000001, 0xffffffff );

	overflowing_sub_test!( u64, 0xffffffff00000000, 0x00000000ffffffff, 0xfffffffe00000001 );
	overflowing_sub_test!( u64, 0x0000000000000000, 0x0000000000000001, 0xffffffffffffffff );

	overflowing_sub_test!( i8, 0x70, 0x0f, 0x61 );
	overflowing_sub_test!( i8, 0x7f, 0xff, 0x80 );
	overflowing_sub_test!( i8, 0x80, 0x01, 0x7f );

	overflowing_sub_test!( i16, 0x7f00, 0x00ff, 0x7e01 );
	overflowing_sub_test!( i16, 0x7fff, 0xffff, 0x8000 );
	overflowing_sub_test!( i16, 0x8000, 0x0001, 0x7fff );

	overflowing_sub_test!( i32, 0x7fff0000, 0x0000ffff, 0x7ffe0001 );
	overflowing_sub_test!( i32, 0x7fffffff, 0xffffffff, 0x80000000 );
	overflowing_sub_test!( i32, 0x80000000, 0x00000001, 0x7fffffff );

	overflowing_sub_test!( i64, 0x7fffffff00000000, 0x00000000ffffffff, 0x7ffffffe00000001 );
	overflowing_sub_test!( i64, 0x7fffffffffffffff, 0xffffffffffffffff, 0x8000000000000000 );
	overflowing_sub_test!( i64, 0x8000000000000000, 0x0000000000000001, 0x7fffffffffffffff );
    }
}
