#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::unchecked_srem;

    use core::i8;
    use core::i16;
    use core::i32;
    use core::i64;
    use core::isize;

    // pub fn unchecked_srem<T>(x: T, y: T) -> T;

    macro_rules! unchecked_srem_test {
	($T:ty, $dividend:expr, $divisor:expr) => ({
	    let x: $T = $dividend;
	    let y: $T = $divisor;
	    let _: $T = unsafe { unchecked_srem::<$T>(x, y) };
	})
    }

    #[test]
    fn unchecked_srem_test1() {
	if false {
	    unchecked_srem_test!(    i8,    i8::MIN, 0 );
	    unchecked_srem_test!(   i16,   i16::MIN, 0 );
	    unchecked_srem_test!(   i32,   i32::MIN, 0 );
	    unchecked_srem_test!(   i64,   i64::MIN, 0 );
	    unchecked_srem_test!( isize, isize::MIN, 0 );

	    unchecked_srem_test!(    i8,    i8::MIN, -1 );
	    unchecked_srem_test!(   i16,   i16::MIN, -1 );
	    unchecked_srem_test!(   i32,   i32::MIN, -1 );
	    unchecked_srem_test!(   i64,   i64::MIN, -1 );
	    unchecked_srem_test!( isize, isize::MIN, -1 );
	}
    }
}
