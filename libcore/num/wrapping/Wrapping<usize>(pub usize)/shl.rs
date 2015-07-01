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
	if cfg!(target_pointer_width = "64") {
	    shl_test!( usize, 0x0000000000000001, 0, 0x0000000000000001 );
	    shl_test!( usize, 0x0000000000000001, 1, 0x0000000000000002 );
	    shl_test!( usize, 0x0000000000000001, 2, 0x0000000000000004 );
	    shl_test!( usize, 0x0000000000000001, 3, 0x0000000000000008 );
	    shl_test!( usize, 0x0000000000000001, 4, 0x0000000000000010 );
	    shl_test!( usize, 0x0000000000000001, 5, 0x0000000000000020 );
	    shl_test!( usize, 0x0000000000000001, 6, 0x0000000000000040 );
	    shl_test!( usize, 0x0000000000000001, 7, 0x0000000000000080 );
	    shl_test!( usize, 0x0000000000000001, 8, 0x0000000000000100 );
	    shl_test!( usize, 0x0000000000000001, 9, 0x0000000000000200 );
	    shl_test!( usize, 0x0000000000000001, 10, 0x0000000000000400 );
	    shl_test!( usize, 0x0000000000000001, 11, 0x0000000000000800 );
	    shl_test!( usize, 0x0000000000000001, 12, 0x0000000000001000 );
	    shl_test!( usize, 0x0000000000000001, 13, 0x0000000000002000 );
	    shl_test!( usize, 0x0000000000000001, 14, 0x0000000000004000 );
	    shl_test!( usize, 0x0000000000000001, 15, 0x0000000000008000 );
	    shl_test!( usize, 0x0000000000000001, 16, 0x0000000000010000 );
	    shl_test!( usize, 0x0000000000000001, 17, 0x0000000000020000 );
	    shl_test!( usize, 0x0000000000000001, 18, 0x0000000000040000 );
	    shl_test!( usize, 0x0000000000000001, 19, 0x0000000000080000 );
	    shl_test!( usize, 0x0000000000000001, 20, 0x0000000000100000 );
	    shl_test!( usize, 0x0000000000000001, 21, 0x0000000000200000 );
	    shl_test!( usize, 0x0000000000000001, 22, 0x0000000000400000 );
	    shl_test!( usize, 0x0000000000000001, 23, 0x0000000000800000 );
	    shl_test!( usize, 0x0000000000000001, 24, 0x0000000001000000 );
	    shl_test!( usize, 0x0000000000000001, 25, 0x0000000002000000 );
	    shl_test!( usize, 0x0000000000000001, 26, 0x0000000004000000 );
	    shl_test!( usize, 0x0000000000000001, 27, 0x0000000008000000 );
	    shl_test!( usize, 0x0000000000000001, 28, 0x0000000010000000 );
	    shl_test!( usize, 0x0000000000000001, 29, 0x0000000020000000 );
	    shl_test!( usize, 0x0000000000000001, 30, 0x0000000040000000 );
	    shl_test!( usize, 0x0000000000000001, 31, 0x0000000080000000 );
	    shl_test!( usize, 0x0000000000000001, 32, 0x0000000100000000 );
	    shl_test!( usize, 0x0000000000000001, 33, 0x0000000200000000 );
	    shl_test!( usize, 0x0000000000000001, 34, 0x0000000400000000 );
	    shl_test!( usize, 0x0000000000000001, 35, 0x0000000800000000 );
	    shl_test!( usize, 0x0000000000000001, 36, 0x0000001000000000 );
	    shl_test!( usize, 0x0000000000000001, 37, 0x0000002000000000 );
	    shl_test!( usize, 0x0000000000000001, 38, 0x0000004000000000 );
	    shl_test!( usize, 0x0000000000000001, 39, 0x0000008000000000 );
	    shl_test!( usize, 0x0000000000000001, 40, 0x0000010000000000 );
	    shl_test!( usize, 0x0000000000000001, 41, 0x0000020000000000 );
	    shl_test!( usize, 0x0000000000000001, 42, 0x0000040000000000 );
	    shl_test!( usize, 0x0000000000000001, 43, 0x0000080000000000 );
	    shl_test!( usize, 0x0000000000000001, 44, 0x0000100000000000 );
	    shl_test!( usize, 0x0000000000000001, 45, 0x0000200000000000 );
	    shl_test!( usize, 0x0000000000000001, 46, 0x0000400000000000 );
	    shl_test!( usize, 0x0000000000000001, 47, 0x0000800000000000 );
	    shl_test!( usize, 0x0000000000000001, 48, 0x0001000000000000 );
	    shl_test!( usize, 0x0000000000000001, 49, 0x0002000000000000 );
	    shl_test!( usize, 0x0000000000000001, 50, 0x0004000000000000 );
	    shl_test!( usize, 0x0000000000000001, 51, 0x0008000000000000 );
	    shl_test!( usize, 0x0000000000000001, 52, 0x0010000000000000 );
	    shl_test!( usize, 0x0000000000000001, 53, 0x0020000000000000 );
	    shl_test!( usize, 0x0000000000000001, 54, 0x0040000000000000 );
	    shl_test!( usize, 0x0000000000000001, 55, 0x0080000000000000 );
	    shl_test!( usize, 0x0000000000000001, 56, 0x0100000000000000 );
	    shl_test!( usize, 0x0000000000000001, 57, 0x0200000000000000 );
	    shl_test!( usize, 0x0000000000000001, 58, 0x0400000000000000 );
	    shl_test!( usize, 0x0000000000000001, 59, 0x0800000000000000 );
	    shl_test!( usize, 0x0000000000000001, 60, 0x1000000000000000 );
	    shl_test!( usize, 0x0000000000000001, 61, 0x2000000000000000 );
	    shl_test!( usize, 0x0000000000000001, 62, 0x4000000000000000 );
	    shl_test!( usize, 0x0000000000000001, 63, 0x8000000000000000 );
	} else {
	    shl_test!( usize, 0x00000001, 0, 0x00000001 );
	    shl_test!( usize, 0x00000001, 1, 0x00000002 );
	    shl_test!( usize, 0x00000001, 2, 0x00000004 );
	    shl_test!( usize, 0x00000001, 3, 0x00000008 );
	    shl_test!( usize, 0x00000001, 4, 0x00000010 );
	    shl_test!( usize, 0x00000001, 5, 0x00000020 );
	    shl_test!( usize, 0x00000001, 6, 0x00000040 );
	    shl_test!( usize, 0x00000001, 7, 0x00000080 );
	    shl_test!( usize, 0x00000001, 8, 0x00000100 );
	    shl_test!( usize, 0x00000001, 9, 0x00000200 );
	    shl_test!( usize, 0x00000001, 10, 0x00000400 );
	    shl_test!( usize, 0x00000001, 11, 0x00000800 );
	    shl_test!( usize, 0x00000001, 12, 0x00001000 );
	    shl_test!( usize, 0x00000001, 13, 0x00002000 );
	    shl_test!( usize, 0x00000001, 14, 0x00004000 );
	    shl_test!( usize, 0x00000001, 15, 0x00008000 );
	    shl_test!( usize, 0x00000001, 16, 0x00010000 );
	    shl_test!( usize, 0x00000001, 17, 0x00020000 );
	    shl_test!( usize, 0x00000001, 18, 0x00040000 );
	    shl_test!( usize, 0x00000001, 19, 0x00080000 );
	    shl_test!( usize, 0x00000001, 20, 0x00100000 );
	    shl_test!( usize, 0x00000001, 21, 0x00200000 );
	    shl_test!( usize, 0x00000001, 22, 0x00400000 );
	    shl_test!( usize, 0x00000001, 23, 0x00800000 );
	    shl_test!( usize, 0x00000001, 24, 0x01000000 );
	    shl_test!( usize, 0x00000001, 25, 0x02000000 );
	    shl_test!( usize, 0x00000001, 26, 0x04000000 );
	    shl_test!( usize, 0x00000001, 27, 0x08000000 );
	    shl_test!( usize, 0x00000001, 28, 0x10000000 );
	    shl_test!( usize, 0x00000001, 29, 0x20000000 );
	    shl_test!( usize, 0x00000001, 30, 0x40000000 );
	    shl_test!( usize, 0x00000001, 31, 0x80000000 );
	}
    }

    #[test]
    #[should_panic]
    fn shl_test2() {
	if cfg!(target_pointer_width = "64") {
	    shl_test!( usize, 0x0000000000000001, 64, 0x0000000000000000 ); // panicked at 'shift operation overflowed'
	} else {
	    shl_test!( usize, 0x00000001, 32, 0x00000000 ); // panicked at 'shift operation overflowed'
	}
    }
}
