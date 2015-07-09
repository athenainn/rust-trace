#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::unchecked_urem;

    use core::u8;
    use core::u16;
    use core::u32;
    use core::u64;
    use core::usize;

    // pub fn unchecked_urem<T>(x: T, y: T) -> T;

    macro_rules! unchecked_urem_test {
	($T:ty, $dividend:expr, $divisor:expr) => ({
	    let x: $T = $dividend;
	    let y: $T = $divisor;
	    let _: $T = unsafe { unchecked_urem::<$T>(x, y) };
	})
    }

    #[test]
    fn unchecked_urem_test1() {
	if false {
	    unchecked_urem_test!(    i8,    i8::MIN, 0 );
	    unchecked_urem_test!(   i16,   i16::MIN, 0 );
	    unchecked_urem_test!(   i32,   i32::MIN, 0 );
	    unchecked_urem_test!(   i64,   i64::MIN, 0 );
	    unchecked_urem_test!( isize, isize::MIN, 0 );

	    unchecked_urem_test!(    i8,    i8::MIN, -1 );
	    unchecked_urem_test!(   i16,   i16::MIN, -1 );
	    unchecked_urem_test!(   i32,   i32::MIN, -1 );
	    unchecked_urem_test!(   i64,   i64::MIN, -1 );
	    unchecked_urem_test!( isize, isize::MIN, -1 );
	}
    }
}
