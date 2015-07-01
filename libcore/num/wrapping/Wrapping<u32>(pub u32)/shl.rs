#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Wrapping;

    use core::ops::Shl;

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

    macro_rules! shl_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let x: $T = $value;
	    let other: usize = $other;
	    let wrapping: Wrapping<$T> = Wrapping::<$T>(x);

	    let result: Wrapping<$T> = wrapping.shl(other);
	    assert_eq!(result.0, $result);

	    let result: Wrapping<$T> = wrapping << other;
	    assert_eq!(result.0, $result);
	})
    }

    #[test]
    fn shl_test1() {
	shl_test!( u32, 0x00000001, 0, 0x00000001 );
	shl_test!( u32, 0x00000001, 1, 0x00000002 );
	shl_test!( u32, 0x00000001, 2, 0x00000004 );
	shl_test!( u32, 0x00000001, 3, 0x00000008 );
	shl_test!( u32, 0x00000001, 4, 0x00000010 );
	shl_test!( u32, 0x00000001, 5, 0x00000020 );
	shl_test!( u32, 0x00000001, 6, 0x00000040 );
	shl_test!( u32, 0x00000001, 7, 0x00000080 );
	shl_test!( u32, 0x00000001, 8, 0x00000100 );
	shl_test!( u32, 0x00000001, 9, 0x00000200 );
	shl_test!( u32, 0x00000001, 10, 0x00000400 );
	shl_test!( u32, 0x00000001, 11, 0x00000800 );
	shl_test!( u32, 0x00000001, 12, 0x00001000 );
	shl_test!( u32, 0x00000001, 13, 0x00002000 );
	shl_test!( u32, 0x00000001, 14, 0x00004000 );
	shl_test!( u32, 0x00000001, 15, 0x00008000 );
	shl_test!( u32, 0x00000001, 16, 0x00010000 );
	shl_test!( u32, 0x00000001, 17, 0x00020000 );
	shl_test!( u32, 0x00000001, 18, 0x00040000 );
	shl_test!( u32, 0x00000001, 19, 0x00080000 );
	shl_test!( u32, 0x00000001, 20, 0x00100000 );
	shl_test!( u32, 0x00000001, 21, 0x00200000 );
	shl_test!( u32, 0x00000001, 22, 0x00400000 );
	shl_test!( u32, 0x00000001, 23, 0x00800000 );
	shl_test!( u32, 0x00000001, 24, 0x01000000 );
	shl_test!( u32, 0x00000001, 25, 0x02000000 );
	shl_test!( u32, 0x00000001, 26, 0x04000000 );
	shl_test!( u32, 0x00000001, 27, 0x08000000 );
	shl_test!( u32, 0x00000001, 28, 0x10000000 );
	shl_test!( u32, 0x00000001, 29, 0x20000000 );
	shl_test!( u32, 0x00000001, 30, 0x40000000 );
	shl_test!( u32, 0x00000001, 31, 0x80000000 );
    }

    #[test]
    #[should_panic]
    fn shl_test2() {
	shl_test!( u32, 0x00000001, 32, 0x00000000 ); // panicked at 'shift operation overflowed'
    }
}
