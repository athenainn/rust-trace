#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Mul;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Mul for A<T> {
	type Output = A<T>;

	fn mul(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value * rhs.value }
	}
    }

    // pub trait Mul<RHS=Self> {
    //     /// The resulting type after applying the `*` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the `*` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn mul(self, rhs: RHS) -> Self::Output;
    // }

    // macro_rules! forward_ref_binop {
    //     (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
    //         #[unstable(feature = "core",
    //                    reason = "recently added, waiting for dust to settle")]
    //         impl<'a> $imp<$u> for &'a $t {
    //             type Output = <$t as $imp<$u>>::Output;
    //
    //             #[inline]
    //             fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
    //                 $imp::$method(*self, other)
    //             }
    //         }
    //
    //         #[unstable(feature = "core",
    //                    reason = "recently added, waiting for dust to settle")]
    //         impl<'a> $imp<&'a $u> for $t {
    //             type Output = <$t as $imp<$u>>::Output;
    //
    //             #[inline]
    //             fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
    //                 $imp::$method(self, *other)
    //             }
    //         }
    //
    //         #[unstable(feature = "core",
    //                    reason = "recently added, waiting for dust to settle")]
    //         impl<'a, 'b> $imp<&'a $u> for &'b $t {
    //             type Output = <$t as $imp<$u>>::Output;
    //
    //             #[inline]
    //             fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
    //                 $imp::$method(*self, *other)
    //             }
    //         }
    //     }
    // }

    // macro_rules! mul_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Mul for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn mul(self, other: $t) -> $t { self * other }
    //         }
    //
    //         forward_ref_binop! { impl Mul, mul for $t, $t }
    //     )*)
    // }

    // mul_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

    type T = i32;

    macro_rules! mul_test {
	($($t:ty)*) => ($({
	    let a: $t = 17 as $t;
	    let b: $t = 4 as $t;
	    {
		let result: $t = a.mul(b);
		assert_eq!(result, 68 as $t);
	    }
	    {
		let result: $t = a * b;
		assert_eq!(result, 68 as $t);
	    }

	    let c: &$t = &(10 as $t);
	    {
		let result: $t = c * b;
		assert_eq!(result, 40 as $t);
	    }
	    {
		let result: $t = b * c;
		assert_eq!(result, 40 as $t);
	    }
	    {
		let result: $t = c * c;
		assert_eq!(result, 100 as $t);
	    }
	})*)
    }

    #[test]
    fn mul_test1() {
	let a: A<T> = A { value: 17 };
	let b: A<T> = A { value: 4 };
	{
	    let result: A<T> = a.mul(b);
	    assert_eq!(result.value, 68);
	}
	{
	    let result: A<T> = a * b;
	    assert_eq!(result.value, 68);
	}
    }

    #[test]
    fn mul_test2() {
	mul_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
