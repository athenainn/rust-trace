#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Add;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Add for A<T> {
	type Output = A<T>;

	fn add(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value + rhs.value }
	}
    }

    // pub trait Add<RHS=Self> {
    //     /// The resulting type after applying the `+` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the `+` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn add(self, rhs: RHS) -> Self::Output;
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

    // macro_rules! add_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Add for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn add(self, other: $t) -> $t { self + other }
    //         }
    //
    //         forward_ref_binop! { impl Add, add for $t, $t }
    //     )*)
    // }

    // add_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

    type T = i32;

    macro_rules! add_test {
	($($t:ty)*) => ($({
	    let a: $t = 68 as $t;
	    let b: $t = 32 as $t;
	    {
		let result: $t = a.add(b);
		assert_eq!(result, 100 as $t);
	    }
	    {
		let result: $t = a + b;
		assert_eq!(result, 100 as $t);
	    }

	    let c: &$t = &(10 as $t);
	    {
		let result: $t = c + a;
		assert_eq!(result, 78 as $t);
	    }
	    {
		let result: $t = a + c;
		assert_eq!(result, 78 as $t);
	    }
	    {
		let result: $t = c + c;
		assert_eq!(result, 20 as $t);
	    }
	})*)
    }

    #[test]
    fn add_test1() {
	let a: A<T> = A { value: 68 };
	let b: A<T> = A { value: 32 };
	{
	    let result: A<T> = a.add(b);
	    assert_eq!(result.value, 100 as T);
	}
	{
	    let result: A<T> = a + b;
	    assert_eq!(result.value, 100 as T);
	}
    }

    #[test]
    fn add_test2() {
	add_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
