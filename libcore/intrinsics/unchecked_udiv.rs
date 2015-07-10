#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::unchecked_udiv;

    use core::u8;
    use core::u16;
    use core::u32;
    use core::u64;
    use core::usize;

    // pub fn unchecked_udiv<T>(x: T, y: T) -> T;

    macro_rules! unchecked_udiv_test {
	($T:ty, $dividend:expr, $divisor:expr) => ({
	    let x: $T = $dividend;
	    let y: $T = $divisor;
	    let _: $T = unsafe { unchecked_udiv::<$T>(x, y) };
	})
    }

    #[test]
    fn unchecked_udiv_test1() {
	if false {
	    unchecked_udiv_test!(    u8,    u8::MIN, 0 );
	    unchecked_udiv_test!(   u16,   u16::MIN, 0 );
	    unchecked_udiv_test!(   u32,   u32::MIN, 0 );
	    unchecked_udiv_test!(   u64,   u64::MIN, 0 );
	    unchecked_udiv_test!( usize, usize::MIN, 0 );
	}
    }
}
