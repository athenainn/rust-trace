#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::overflowing_add;

    // pub fn overflowing_add<T>(a: T, b: T) -> T;

    macro_rules! overflowing_add_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let value: $T = $value;
	    let other: $T = $other;
	    let result: $T = unsafe { overflowing_add(value, other) };

	    assert_eq!(result, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn overflowing_add_test1() {
	overflowing_add_test!( u8, 0xf0, 0x0f, 0xff );
	overflowing_add_test!( u8, 0xff, 0x01, 0x00 );

	overflowing_add_test!( u16, 0xff00, 0x00ff, 0xffff );
	overflowing_add_test!( u16, 0xffff, 0x0001, 0x0000 );

	overflowing_add_test!( u32, 0xffff0000, 0x0000ffff, 0xffffffff );
	overflowing_add_test!( u32, 0xffffffff, 0x00000001, 0x00000000 );

	overflowing_add_test!( u64, 0xffffffff00000000, 0x00000000ffffffff, 0xffffffffffffffff );
	overflowing_add_test!( u64, 0xffffffffffffffff, 0x0000000000000001, 0x0000000000000000 );

	overflowing_add_test!( i8, 0x70, 0x0f, 0x7f );
	overflowing_add_test!( i8, 0x7f, 0x01, 0x80 );
	overflowing_add_test!( i8, 0x80, 0xff, 0x7f );

	overflowing_add_test!( i16, 0x7f00, 0x00ff, 0x7fff );
	overflowing_add_test!( i16, 0x7fff, 0x0001, 0x8000 );
	overflowing_add_test!( i16, 0x8000, 0xffff, 0x7fff );

	overflowing_add_test!( i32, 0x7fff0000, 0x0000ffff, 0x7fffffff );
	overflowing_add_test!( i32, 0x7fffffff, 0x00000001, 0x80000000 );
	overflowing_add_test!( i32, 0x80000000, 0xffffffff, 0x7fffffff );

	overflowing_add_test!( i64, 0x7fffffff00000000, 0x00000000ffffffff, 0x7fffffffffffffff );
	overflowing_add_test!( i64, 0x7fffffffffffffff, 0x0000000000000001, 0x8000000000000000 );
	overflowing_add_test!( i64, 0x8000000000000000, 0xffffffffffffffff, 0x7fffffffffffffff );
    }
}
