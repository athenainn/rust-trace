#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Div;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Div for A<T> {
	type Output = A<T>;

	fn div(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value / rhs.value }
	}
    }

    // pub trait Div<RHS=Self> {
    //     /// The resulting type after applying the `/` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the `/` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn div(self, rhs: RHS) -> Self::Output;
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

    // macro_rules! div_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Div for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn div(self, other: $t) -> $t { self / other }
    //         }
    //
    //         forward_ref_binop! { impl Div, div for $t, $t }
    //     )*)
    // }

    // div_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

    type T = i32;

    macro_rules! div_test {
	($($t:ty)*) => ($({
	    let a: $t = 68 as $t;
	    let b: $t = 4 as $t;
	    {
		let result: $t = a.div(b);
		assert_eq!(result, 17 as $t);
	    }
	    {
		let result: $t = a / b;
		assert_eq!(result, 17 as $t);
	    }

	    let c: &$t = &(4 as $t);
	    {
		let result: $t = c / b;
		assert_eq!(result, 1 as $t);
	    }
	    {
		let result: $t = a / c;
		assert_eq!(result, 17 as $t);
	    }
	    {
		let result: $t = c / c;
		assert_eq!(result, 1 as $t);
	    }
	})*)
    }

    #[test]
    fn div_test1() {
	let a: A<T> = A { value: 68 };
	let b: A<T> = A { value: 4 };
	{
	    let result: A<T> = a.div(b);
	    assert_eq!(result.value, 17);
	}
	{
	    let result: A<T> = a / b;
	    assert_eq!(result.value, 17);
	}
    }

    #[test]
    fn div_test2() {
	div_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
