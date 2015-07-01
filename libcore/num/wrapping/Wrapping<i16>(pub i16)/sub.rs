#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::Wrapping;

    use core::ops::Sub;

    // macro_rules! wrapping_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Add for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn add(self, other: Wrapping<$t>) -> Wrapping<$t> {
    //                 Wrapping(self.0.wrapping_add(other.0))
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Sub for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn sub(self, other: Wrapping<$t>) -> Wrapping<$t> {
    //                 Wrapping(self.0.wrapping_sub(other.0))
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Mul for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn mul(self, other: Wrapping<$t>) -> Wrapping<$t> {
    //                 Wrapping(self.0.wrapping_mul(other.0))
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Not for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn not(self) -> Wrapping<$t> {
    //                 Wrapping(!self.0)
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl BitXor for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn bitxor(self, other: Wrapping<$t>) -> Wrapping<$t> {
    //                 Wrapping(self.0 ^ other.0)
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl BitOr for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn bitor(self, other: Wrapping<$t>) -> Wrapping<$t> {
    //                 Wrapping(self.0 | other.0)
    //             }
    //         }
    //
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl BitAnd for Wrapping<$t> {
    //             type Output = Wrapping<$t>;
    //
    //             #[inline(always)]
    //             fn bitand(self, other: Wrapping<$t>) -> Wrapping<$t> {
    //                 Wrapping(self.0 & other.0)
    //             }
    //         }
    //     )*)
    // }

    // wrapping_impl! { usize u8 u16 u32 u64 isize i8 i16 i64 i64 }

    macro_rules! sub_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let value: $T = $value;
	    let wrapping_value: Wrapping<$T> = Wrapping::<$T>(value);
	    let other: $T = $other;
	    let wrapping_other: Wrapping<$T> = Wrapping::<$T>(other);

	    let result: Wrapping<$T> = wrapping_value.sub(wrapping_other);
	    assert_eq!(result.0, $result);

	    let result: Wrapping<$T> = wrapping_value - wrapping_other;
	    assert_eq!(result.0, $result);
	})
    }

    #[test]
    fn sub_test1() {
	sub_test!( i16, 0x7f00, 0x00ff, 0x7e01 );
    }

    #[test]
    #[allow(overflowing_literals)]
    fn sub_test2() {
	    sub_test!( i16, 0x8000, 0x0001, 0x7fff );
    }

    #[test]
    #[allow(overflowing_literals)]
    fn sub_test3() {
	    sub_test!( i16, 0x7fff, 0xffff, 0x8000 );
    }
}
