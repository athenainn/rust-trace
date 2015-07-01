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
	shr_test!( i8, 0x80, 0, 0x80 );
	shr_test!( i8, 0x80, 1, 0xc0 );
	shr_test!( i8, 0x80, 2, 0xe0 );
	shr_test!( i8, 0x80, 3, 0xf0 );
	shr_test!( i8, 0x80, 4, 0xf8 );
	shr_test!( i8, 0x80, 5, 0xfc );
	shr_test!( i8, 0x80, 6, 0xfe );
	shr_test!( i8, 0x80, 7, 0xff );
    }

    #[test]
    #[allow(overflowing_literals)]
    #[should_panic]
    fn shr_test2() {
	shr_test!( i8, 0x80, 8, 0xff ); // panicked at 'shift operation overflowed'
    }
}
