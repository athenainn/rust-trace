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
	shr_test!( i32, 0x80000000, 0, 0x80000000 );
	shr_test!( i32, 0x80000000, 1, 0xc0000000 );
	shr_test!( i32, 0x80000000, 2, 0xe0000000 );
	shr_test!( i32, 0x80000000, 3, 0xf0000000 );
	shr_test!( i32, 0x80000000, 4, 0xf8000000 );
	shr_test!( i32, 0x80000000, 5, 0xfc000000 );
	shr_test!( i32, 0x80000000, 6, 0xfe000000 );
	shr_test!( i32, 0x80000000, 7, 0xff000000 );
	shr_test!( i32, 0x80000000, 8, 0xff800000 );
	shr_test!( i32, 0x80000000, 9, 0xffc00000 );
	shr_test!( i32, 0x80000000, 10, 0xffe00000 );
	shr_test!( i32, 0x80000000, 11, 0xfff00000 );
	shr_test!( i32, 0x80000000, 12, 0xfff80000 );
	shr_test!( i32, 0x80000000, 13, 0xfffc0000 );
	shr_test!( i32, 0x80000000, 14, 0xfffe0000 );
	shr_test!( i32, 0x80000000, 15, 0xffff0000 );
	shr_test!( i32, 0x80000000, 16, 0xffff8000 );
	shr_test!( i32, 0x80000000, 17, 0xffffc000 );
	shr_test!( i32, 0x80000000, 18, 0xffffe000 );
	shr_test!( i32, 0x80000000, 19, 0xfffff000 );
	shr_test!( i32, 0x80000000, 20, 0xfffff800 );
	shr_test!( i32, 0x80000000, 21, 0xfffffc00 );
	shr_test!( i32, 0x80000000, 22, 0xfffffe00 );
	shr_test!( i32, 0x80000000, 23, 0xffffff00 );
	shr_test!( i32, 0x80000000, 24, 0xffffff80 );
	shr_test!( i32, 0x80000000, 25, 0xffffffc0 );
	shr_test!( i32, 0x80000000, 26, 0xffffffe0 );
	shr_test!( i32, 0x80000000, 27, 0xfffffff0 );
	shr_test!( i32, 0x80000000, 28, 0xfffffff8 );
	shr_test!( i32, 0x80000000, 29, 0xfffffffc );
	shr_test!( i32, 0x80000000, 30, 0xfffffffe );
	shr_test!( i32, 0x80000000, 31, 0xffffffff );
    }

    #[test]
    #[allow(overflowing_literals)]
    #[should_panic]
    fn shr_test2() {
	shr_test!( i32, 0x80000000, 32, 0xffffffff ); // panicked at 'shift operation overflowed'
    }
}
