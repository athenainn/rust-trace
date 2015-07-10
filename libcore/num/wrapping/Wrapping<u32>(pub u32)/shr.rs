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
    fn shr_test1() {
	shr_test!( u32, 0x80000000, 0, 0x80000000 );
	shr_test!( u32, 0x80000000, 1, 0x40000000 );
	shr_test!( u32, 0x80000000, 2, 0x20000000 );
	shr_test!( u32, 0x80000000, 3, 0x10000000 );
	shr_test!( u32, 0x80000000, 4, 0x08000000 );
	shr_test!( u32, 0x80000000, 5, 0x04000000 );
	shr_test!( u32, 0x80000000, 6, 0x02000000 );
	shr_test!( u32, 0x80000000, 7, 0x01000000 );
	shr_test!( u32, 0x80000000, 8, 0x00800000 );
	shr_test!( u32, 0x80000000, 9, 0x00400000 );
	shr_test!( u32, 0x80000000, 10, 0x00200000 );
	shr_test!( u32, 0x80000000, 11, 0x00100000 );
	shr_test!( u32, 0x80000000, 12, 0x00080000 );
	shr_test!( u32, 0x80000000, 13, 0x00040000 );
	shr_test!( u32, 0x80000000, 14, 0x00020000 );
	shr_test!( u32, 0x80000000, 15, 0x00010000 );
	shr_test!( u32, 0x80000000, 16, 0x00008000 );
	shr_test!( u32, 0x80000000, 17, 0x00004000 );
	shr_test!( u32, 0x80000000, 18, 0x00002000 );
	shr_test!( u32, 0x80000000, 19, 0x00001000 );
	shr_test!( u32, 0x80000000, 20, 0x00000800 );
	shr_test!( u32, 0x80000000, 21, 0x00000400 );
	shr_test!( u32, 0x80000000, 22, 0x00000200 );
	shr_test!( u32, 0x80000000, 23, 0x00000100 );
	shr_test!( u32, 0x80000000, 24, 0x00000080 );
	shr_test!( u32, 0x80000000, 25, 0x00000040 );
	shr_test!( u32, 0x80000000, 26, 0x00000020 );
	shr_test!( u32, 0x80000000, 27, 0x00000010 );
	shr_test!( u32, 0x80000000, 28, 0x00000008 );
	shr_test!( u32, 0x80000000, 29, 0x00000004 );
	shr_test!( u32, 0x80000000, 30, 0x00000002 );
	shr_test!( u32, 0x80000000, 31, 0x00000001 );
    }

    #[test]
    #[should_panic]
    fn shr_test2() {
	shr_test!( u32, 0x80000000, 32, 0x00000000 ); // panicked at 'shift operation overflowed'
    }
}
