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
    fn shr_test1() {
	shr_test!( u16, 0x8000, 0, 0x8000 );
	shr_test!( u16, 0x8000, 1, 0x4000 );
	shr_test!( u16, 0x8000, 2, 0x2000 );
	shr_test!( u16, 0x8000, 3, 0x1000 );
	shr_test!( u16, 0x8000, 4, 0x0800 );
	shr_test!( u16, 0x8000, 5, 0x0400 );
	shr_test!( u16, 0x8000, 6, 0x0200 );
	shr_test!( u16, 0x8000, 7, 0x0100 );
	shr_test!( u16, 0x8000, 8, 0x0080 );
	shr_test!( u16, 0x8000, 9, 0x0040 );
	shr_test!( u16, 0x8000, 10, 0x0020 );
	shr_test!( u16, 0x8000, 11, 0x0010 );
	shr_test!( u16, 0x8000, 12, 0x0008 );
	shr_test!( u16, 0x8000, 13, 0x0004 );
	shr_test!( u16, 0x8000, 14, 0x0002 );
	shr_test!( u16, 0x8000, 15, 0x0001 );
    }

    #[test]
    #[should_panic]
    fn shr_test2() {
	shr_test!( u16, 0x8000, 16, 0x0000 ); // panicked at 'shift operation overflowed'
    }
}
