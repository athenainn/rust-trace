#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Rem;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Rem for A<T> {
	type Output = A<T>;

	fn rem(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value % rhs.value }
	}
    }

    // pub trait Rem<RHS=Self> {
    //     /// The resulting type after applying the `%` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output = Self;
    //
    //     /// The method for the `%` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn rem(self, rhs: RHS) -> Self::Output;
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

    // macro_rules! rem_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Rem for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn rem(self, other: $t) -> $t { self % other }
    //         }
    //
    //         forward_ref_binop! { impl Rem, rem for $t, $t }
    //     )*)
    // }

    // macro_rules! rem_float_impl {
    //     ($t:ty, $fmod:ident) => {
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Rem for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn rem(self, other: $t) -> $t {
    //                 extern { fn $fmod(a: $t, b: $t) -> $t; }
    //                 unsafe { $fmod(self, other) }
    //             }
    //         }
    //
    //         forward_ref_binop! { impl Rem, rem for $t, $t }
    //     }
    // }

    // rem_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 }
    // rem_float_impl! { f32, fmodf }
    // rem_float_impl! { f64, fmod }

    type T = i32;

    macro_rules! rem_test {
	($($t:ty)*) => ($({
	    let a: $t = 68 as $t;
	    let b: $t = 11 as $t;
	    {
		let result: $t = a.rem(b);
		assert_eq!(result, 2 as $t);
	    }
	    {
		let result: $t = a % b;
		assert_eq!(result, 2 as $t);
	    }

	    let c: &$t = &(4 as $t);
	    {
		let result: $t = c % b;
		assert_eq!(result, 4 as $t);
	    }
	    {
		let result: $t = a % c;
		assert_eq!(result, 0 as $t);
	    }
	    {
		let result: $t = c % c;
		assert_eq!(result, 0 as $t);
	    }
	})*)
    }

    #[test]
    fn rem_test1() {
	let a: A<T> = A { value: 68 };
	let b: A<T> = A { value: 11 };
	{
	    let result: A<T> = a.rem(b);
	    assert_eq!(result.value, 2);
	}
	{
	    let result: A<T> = a % b;
	    assert_eq!(result.value, 2);
	}
    }

    #[test]
    fn rem_test2() {
	rem_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
