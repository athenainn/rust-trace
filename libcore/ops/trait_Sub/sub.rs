#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Sub;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Sub for A<T> {
	type Output = A<T>;

	fn sub(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value - rhs.value }
	}
    }

    // pub trait Sub<RHS=Self> {
    //     /// The resulting type after applying the `-` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the `-` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn sub(self, rhs: RHS) -> Self::Output;
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

    // macro_rules! sub_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Sub for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn sub(self, other: $t) -> $t { self - other }
    //         }
    //
    //         forward_ref_binop! { impl Sub, sub for $t, $t }
    //     )*)
    // }

    // sub_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

    type T = i32;

    macro_rules! sub_test {
	($($t:ty)*) => ($({
	    let a: $t = 68 as $t;
	    let b: $t = 32 as $t;
	    {
		let result: $t = a.sub(b);
		assert_eq!(result, 36 as $t);
	    }
	    {
		let result: $t = a - b;
		assert_eq!(result, 36 as $t);
	    }

	    let c: &$t = &(50 as $t);
	    {
		let result: $t = c - b;
		assert_eq!(result, 18 as $t);
	    }
	    {
		let result: $t = a - c;
		assert_eq!(result, 18 as $t);
	    }
	    {
		let result: $t = c - c;
		assert_eq!(result, 0 as $t);
	    }
	})*)
    }

    #[test]
    fn sub_test1() {
	let a: A<T> = A { value: 68 };
	let b: A<T> = A { value: 32 };
	{
	    let result: A<T> = a.sub(b);
	    assert_eq!(result.value, 36);
	}
	{
	    let result: A<T> = a - b;
	    assert_eq!(result.value, 36);
	}
    }

    #[test]
    fn sub_test2() {
	sub_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
