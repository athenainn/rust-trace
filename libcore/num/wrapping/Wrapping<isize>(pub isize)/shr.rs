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

    // sh_impl_all! { u8 u32 u32 u64 usize i8 i16 i32 i64 isize }

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
	if cfg!(target_pointer_width = "64") {
	    shr_test!( isize, 0x8000000000000000, 0, 0x8000000000000000 );
	    shr_test!( isize, 0x8000000000000000, 1, 0xc000000000000000 );
	    shr_test!( isize, 0x8000000000000000, 2, 0xe000000000000000 );
	    shr_test!( isize, 0x8000000000000000, 3, 0xf000000000000000 );
	    shr_test!( isize, 0x8000000000000000, 4, 0xf800000000000000 );
	    shr_test!( isize, 0x8000000000000000, 5, 0xfc00000000000000 );
	    shr_test!( isize, 0x8000000000000000, 6, 0xfe00000000000000 );
	    shr_test!( isize, 0x8000000000000000, 7, 0xff00000000000000 );
	    shr_test!( isize, 0x8000000000000000, 8, 0xff80000000000000 );
	    shr_test!( isize, 0x8000000000000000, 9, 0xffc0000000000000 );
	    shr_test!( isize, 0x8000000000000000, 10, 0xffe0000000000000 );
	    shr_test!( isize, 0x8000000000000000, 11, 0xfff0000000000000 );
	    shr_test!( isize, 0x8000000000000000, 12, 0xfff8000000000000 );
	    shr_test!( isize, 0x8000000000000000, 13, 0xfffc000000000000 );
	    shr_test!( isize, 0x8000000000000000, 14, 0xfffe000000000000 );
	    shr_test!( isize, 0x8000000000000000, 15, 0xffff000000000000 );
	    shr_test!( isize, 0x8000000000000000, 16, 0xffff800000000000 );
	    shr_test!( isize, 0x8000000000000000, 17, 0xffffc00000000000 );
	    shr_test!( isize, 0x8000000000000000, 18, 0xffffe00000000000 );
	    shr_test!( isize, 0x8000000000000000, 19, 0xfffff00000000000 );
	    shr_test!( isize, 0x8000000000000000, 20, 0xfffff80000000000 );
	    shr_test!( isize, 0x8000000000000000, 21, 0xfffffc0000000000 );
	    shr_test!( isize, 0x8000000000000000, 22, 0xfffffe0000000000 );
	    shr_test!( isize, 0x8000000000000000, 23, 0xffffff0000000000 );
	    shr_test!( isize, 0x8000000000000000, 24, 0xffffff8000000000 );
	    shr_test!( isize, 0x8000000000000000, 25, 0xffffffc000000000 );
	    shr_test!( isize, 0x8000000000000000, 26, 0xffffffe000000000 );
	    shr_test!( isize, 0x8000000000000000, 27, 0xfffffff000000000 );
	    shr_test!( isize, 0x8000000000000000, 28, 0xfffffff800000000 );
	    shr_test!( isize, 0x8000000000000000, 29, 0xfffffffc00000000 );
	    shr_test!( isize, 0x8000000000000000, 30, 0xfffffffe00000000 );
	    shr_test!( isize, 0x8000000000000000, 31, 0xffffffff00000000 );
	    shr_test!( isize, 0x8000000000000000, 32, 0xffffffff80000000 );
	    shr_test!( isize, 0x8000000000000000, 33, 0xffffffffc0000000 );
	    shr_test!( isize, 0x8000000000000000, 34, 0xffffffffe0000000 );
	    shr_test!( isize, 0x8000000000000000, 35, 0xfffffffff0000000 );
	    shr_test!( isize, 0x8000000000000000, 36, 0xfffffffff8000000 );
	    shr_test!( isize, 0x8000000000000000, 37, 0xfffffffffc000000 );
	    shr_test!( isize, 0x8000000000000000, 38, 0xfffffffffe000000 );
	    shr_test!( isize, 0x8000000000000000, 39, 0xffffffffff000000 );
	    shr_test!( isize, 0x8000000000000000, 40, 0xffffffffff800000 );
	    shr_test!( isize, 0x8000000000000000, 41, 0xffffffffffc00000 );
	    shr_test!( isize, 0x8000000000000000, 42, 0xffffffffffe00000 );
	    shr_test!( isize, 0x8000000000000000, 43, 0xfffffffffff00000 );
	    shr_test!( isize, 0x8000000000000000, 44, 0xfffffffffff80000 );
	    shr_test!( isize, 0x8000000000000000, 45, 0xfffffffffffc0000 );
	    shr_test!( isize, 0x8000000000000000, 46, 0xfffffffffffe0000 );
	    shr_test!( isize, 0x8000000000000000, 47, 0xffffffffffff0000 );
	    shr_test!( isize, 0x8000000000000000, 48, 0xffffffffffff8000 );
	    shr_test!( isize, 0x8000000000000000, 49, 0xffffffffffffc000 );
	    shr_test!( isize, 0x8000000000000000, 50, 0xffffffffffffe000 );
	    shr_test!( isize, 0x8000000000000000, 51, 0xfffffffffffff000 );
	    shr_test!( isize, 0x8000000000000000, 52, 0xfffffffffffff800 );
	    shr_test!( isize, 0x8000000000000000, 53, 0xfffffffffffffc00 );
	    shr_test!( isize, 0x8000000000000000, 54, 0xfffffffffffffe00 );
	    shr_test!( isize, 0x8000000000000000, 55, 0xffffffffffffff00 );
	    shr_test!( isize, 0x8000000000000000, 56, 0xffffffffffffff80 );
	    shr_test!( isize, 0x8000000000000000, 57, 0xffffffffffffffc0 );
	    shr_test!( isize, 0x8000000000000000, 58, 0xffffffffffffffe0 );
	    shr_test!( isize, 0x8000000000000000, 59, 0xfffffffffffffff0 );
	    shr_test!( isize, 0x8000000000000000, 60, 0xfffffffffffffff8 );
	    shr_test!( isize, 0x8000000000000000, 61, 0xfffffffffffffffc );
	    shr_test!( isize, 0x8000000000000000, 62, 0xfffffffffffffffe );
	    shr_test!( isize, 0x8000000000000000, 63, 0xffffffffffffffff );
	} else {
	    shr_test!( isize, 0x80000000, 0, 0x80000000 );
	    shr_test!( isize, 0x80000000, 1, 0x40000000 );
	    shr_test!( isize, 0x80000000, 2, 0x20000000 );
	    shr_test!( isize, 0x80000000, 3, 0x10000000 );
	    shr_test!( isize, 0x80000000, 4, 0xf8000000 );
	    shr_test!( isize, 0x80000000, 5, 0xf4000000 );
	    shr_test!( isize, 0x80000000, 6, 0xf2000000 );
	    shr_test!( isize, 0x80000000, 7, 0xf1000000 );
	    shr_test!( isize, 0x80000000, 8, 0xff800000 );
	    shr_test!( isize, 0x80000000, 9, 0xff400000 );
	    shr_test!( isize, 0x80000000, 10, 0xff200000 );
	    shr_test!( isize, 0x80000000, 11, 0xff100000 );
	    shr_test!( isize, 0x80000000, 12, 0xfff80000 );
	    shr_test!( isize, 0x80000000, 13, 0xfff40000 );
	    shr_test!( isize, 0x80000000, 14, 0xfff20000 );
	    shr_test!( isize, 0x80000000, 15, 0xfff10000 );
	    shr_test!( isize, 0x80000000, 16, 0xffff8000 );
	    shr_test!( isize, 0x80000000, 17, 0xffff4000 );
	    shr_test!( isize, 0x80000000, 18, 0xffff2000 );
	    shr_test!( isize, 0x80000000, 19, 0xffff1000 );
	    shr_test!( isize, 0x80000000, 20, 0xfffff800 );
	    shr_test!( isize, 0x80000000, 21, 0xfffff400 );
	    shr_test!( isize, 0x80000000, 22, 0xfffff200 );
	    shr_test!( isize, 0x80000000, 23, 0xfffff100 );
	    shr_test!( isize, 0x80000000, 24, 0xffffff80 );
	    shr_test!( isize, 0x80000000, 25, 0xffffff40 );
	    shr_test!( isize, 0x80000000, 26, 0xffffff20 );
	    shr_test!( isize, 0x80000000, 27, 0xffffff10 );
	    shr_test!( isize, 0x80000000, 28, 0xfffffff8 );
	    shr_test!( isize, 0x80000000, 29, 0xfffffff4 );
	    shr_test!( isize, 0x80000000, 30, 0xfffffff2 );
	    shr_test!( isize, 0x80000000, 31, 0xfffffff1 );
	}
    }

    #[test]
    #[allow(overflowing_literals)]
    #[should_panic]
    fn shr_test2() {
	if cfg!(target_pointer_width = "64") {
	    shr_test!( isize, 0x8000000000000000, 64, 0xffffffffffffffff ); // panicked at 'shift operation overflowed'
	} else {
	    shr_test!( isize, 0x80000000, 32, 0xffffffff ); // panicked at 'shift operation overflowed'
	}
    }
}
