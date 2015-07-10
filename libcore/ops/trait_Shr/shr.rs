#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Shr;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Shr<A<T>> for A<T> {
	type Output = A<T>;

	fn shr(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value >> rhs.value }
	}
    }

    // pub trait Shr<RHS> {
    //     /// The resulting type after applying the `>>` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the `>>` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn shr(self, rhs: RHS) -> Self::Output;
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

    // macro_rules! shr_impl {
    //     ($t:ty, $f:ty) => (
    //         impl Shr<$f> for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn shr(self, other: $f) -> $t {
    //                 self >> other
    //             }
    //         }
    //
    //         forward_ref_binop! { impl Shr, shr for $t, $f }
    //     )
    // }

    // macro_rules! shr_impl_all {
    //     ($($t:ty)*) => ($(
    //         shr_impl! { $t, u8 }
    //         shr_impl! { $t, u16 }
    //         shr_impl! { $t, u32 }
    //         shr_impl! { $t, u64 }
    //         shr_impl! { $t, usize }
    //
    //         shr_impl! { $t, i8 }
    //         shr_impl! { $t, i16 }
    //         shr_impl! { $t, i32 }
    //         shr_impl! { $t, i64 }
    //         shr_impl! { $t, isize }
    //     )*)
    // }

    // shr_impl_all! { u8 u16 u32 u64 usize i8 i16 i32 i64 isize }

    type T = i32;

    macro_rules! shr_test {
	($t:ty, $f:ty) => ({
	    let a: $t = 0x40 as $t;
	    let b: $f = 0x2 as $f;
	    {
		let result: $t = a.shr(b);
		assert_eq!(result, 0x10 as $t);
	    }
	    {
		let result: $t = a >> b;
		assert_eq!(result, 0x10 as $t);
	    }

	    let c: &$t = &(0x4 as $t);
	    {
		let result: $t = c >> b;
		assert_eq!(result, 0x1 as $t);
	    }
	    {
		let result: $t = a >> c;
		assert_eq!(result, 0x4 as $t);
	    }

	    let d: &$t = &(0x1 as $t);
	    {
		let result: $t = c >> d;
		assert_eq!(result, 0x2 as $t);
	    }
	})
    }

    macro_rules! shr_all_test {
	($($t:ty)*) => ($(
	    shr_test! { $t, u8 }
	    shr_test! { $t, u16 }
	    shr_test! { $t, u32 }
	    shr_test! { $t, u64 }
	    shr_test! { $t, usize }

	    shr_test! { $t, i8 }
	    shr_test! { $t, i16 }
	    shr_test! { $t, i32 }
	    shr_test! { $t, i64 }
	    shr_test! { $t, isize }
	)*)
    }

    #[test]
    fn shr_test1() {
	let a: A<T> = A { value: 0x40 };
	let b: A<T> = A { value: 0x3 };
	{
	    let result: A<T> = a.shr(b);
	    assert_eq!(result.value, 0x8 as T);
	}
	{
	    let result: A<T> = a >> b;
	    assert_eq!(result.value, 0x8 as T);
	}
    }

    #[test]
    fn shr_test2() {
	shr_all_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 };
    }
}
