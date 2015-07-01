#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Wrapping;

    use core::ops::Shr;

    // macro_rules! sh_impl {
    //     ($t:ty, $f:ty) => (
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Shl<$f> for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn shl(self, other: $f) -> Wrapping<$t> {
    //                 Wrapping(self.0 << other)
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Shr<$f> for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn shr(self, other: $f) -> Wrapping<$t> {
    //                 Wrapping(self.0 >> other)
    //             }
    //         }
    //     )
    // }

    // macro_rules! sh_impl_all {
    //     ($($t:ty)*) => ($(
    //         // sh_impl! { $t, u8 }
    //         // sh_impl! { $t, u16 }
    //         // sh_impl! { $t, u32 }
    //         // sh_impl! { $t, u64 }
    //         sh_impl! { $t, usize }
    //
    //         // sh_impl! { $t, i8 }
    //         // sh_impl! { $t, i16 }
    //         // sh_impl! { $t, i32 }
    //         // sh_impl! { $t, i64 }
    //         // sh_impl! { $t, isize }
    //     )*)
    // }

    // sh_impl_all! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

    macro_rules! shr_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let x: $T = $value;
	    let other: usize = $other;
	    let wrapping: Wrapping<$T> = Wrapping::<$T>(x);

	    let result: Wrapping<$T> = wrapping.shr(other);
	    assert_eq!(result.0, $result);

	    let result: Wrapping<$T> = wrapping >> other;
	    assert_eq!(result.0, $result);
	})
    }

    #[test]
    #[allow(overflowing_literals)]
    fn shr_test1() {
	shr_test!( i16, 0x8000, 0, 0x8000 );
	shr_test!( i16, 0x8000, 1, 0xc000 );
	shr_test!( i16, 0x8000, 2, 0xe000 );
	shr_test!( i16, 0x8000, 3, 0xf000 );
	shr_test!( i16, 0x8000, 4, 0xf800 );
	shr_test!( i16, 0x8000, 5, 0xfc00 );
	shr_test!( i16, 0x8000, 6, 0xfe00 );
	shr_test!( i16, 0x8000, 7, 0xff00 );
	shr_test!( i16, 0x8000, 8, 0xff80 );
	shr_test!( i16, 0x8000, 9, 0xffc0 );
	shr_test!( i16, 0x8000, 10, 0xffe0 );
	shr_test!( i16, 0x8000, 11, 0xfff0 );
	shr_test!( i16, 0x8000, 12, 0xfff8 );
	shr_test!( i16, 0x8000, 13, 0xfffc );
	shr_test!( i16, 0x8000, 14, 0xfffe );
	shr_test!( i16, 0x8000, 15, 0xffff );
    }

    #[test]
    #[allow(overflowing_literals)]
    #[should_panic]
    fn shr_test2() {
	shr_test!( i16, 0x8000, 16, 0xffff ); // panicked at 'shift operation overflowed'
    }
}
