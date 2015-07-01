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
	shl_test!( u16, 0x0001, 0, 0x0001 );
	shl_test!( u16, 0x0001, 1, 0x0002 );
	shl_test!( u16, 0x0001, 2, 0x0004 );
	shl_test!( u16, 0x0001, 3, 0x0008 );
	shl_test!( u16, 0x0001, 4, 0x0010 );
	shl_test!( u16, 0x0001, 5, 0x0020 );
	shl_test!( u16, 0x0001, 6, 0x0040 );
	shl_test!( u16, 0x0001, 7, 0x0080 );
	shl_test!( u16, 0x0001, 8, 0x0100 );
	shl_test!( u16, 0x0001, 9, 0x0200 );
	shl_test!( u16, 0x0001, 10, 0x0400 );
	shl_test!( u16, 0x0001, 11, 0x0800 );
	shl_test!( u16, 0x0001, 12, 0x1000 );
	shl_test!( u16, 0x0001, 13, 0x2000 );
	shl_test!( u16, 0x0001, 14, 0x4000 );
	shl_test!( u16, 0x0001, 15, 0x8000 );
    }

    #[test]
    #[should_panic]
    fn shl_test2() {
	shl_test!( u16, 0x0001, 16, 0x0000 ); // panicked at 'shift operation overflowed'
    }
}
