#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::unchecked_sdiv;

    use core::i8;
    use core::i16;
    use core::i32;
    use core::i64;
    use core::isize;

    // pub fn unchecked_sdiv<T>(x: T, y: T) -> T;

    macro_rules! unchecked_sdiv_test {
	($T:ty, $dividend:expr, $divisor:expr) => ({
	    let x: $T = $dividend;
	    let y: $T = $divisor;
	    let _: $T = unsafe { unchecked_sdiv::<$T>(x, y) };
	})
    }

    #[test]
    fn unchecked_sdiv_test1() {
	if false {
	    unchecked_sdiv_test!(    i8,    i8::MIN, 0 );
	    unchecked_sdiv_test!(   i16,   i16::MIN, 0 );
	    unchecked_sdiv_test!(   i32,   i32::MIN, 0 );
	    unchecked_sdiv_test!(   i64,   i64::MIN, 0 );
	    unchecked_sdiv_test!( isize, isize::MIN, 0 );

	    unchecked_sdiv_test!(    i8,    i8::MIN, -1 );
	    unchecked_sdiv_test!(   i16,   i16::MIN, -1 );
	    unchecked_sdiv_test!(   i32,   i32::MIN, -1 );
	    unchecked_sdiv_test!(   i64,   i64::MIN, -1 );
	    unchecked_sdiv_test!( isize, isize::MIN, -1 );
	}
    }
}
