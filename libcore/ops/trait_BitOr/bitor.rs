#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::BitOr;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl BitOr for A<T> {
	type Output = A<T>;

	fn bitor(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value | rhs.value }
	}
    }

    // pub trait BitOr<RHS=Self> {
    //     /// The resulting type after applying the `|` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the `|` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn bitor(self, rhs: RHS) -> Self::Output;
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

    // macro_rules! bitor_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl BitOr for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn bitor(self, rhs: $t) -> $t { self | rhs }
    //         }
    //
    //         forward_ref_binop! { impl BitOr, bitor for $t, $t }
    //     )*)
    // }

    // bitor_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

    type T = i32;

    macro_rules! bitor_test {
	($($t:ty)*) => ($({
	    let a: $t = 0x70 as $t;
	    let b: $t = 0xf as $t;
	    {
		let result: $t = a.bitor(b);
		assert_eq!(result, 0x7f as $t);
	    }
	    {
		let result: $t = a | b;
		assert_eq!(result, 0x7f as $t);
	    }

	    let c: &$t = &(0xf as $t);
	    {
		let result: $t = c | a;
		assert_eq!(result, 0x7f as $t);
	    }
	    {
		let result: $t = a | c;
		assert_eq!(result, 0x7f as $t);
	    }
	    {
		let result: $t = c | c;
		assert_eq!(result, 0xf as $t);
	    }
	})*)
    }

    #[test]
    fn bitor_test1() {
	let a: A<T> = A { value: 0x70 };
	let b: A<T> = A { value: 0xf };
	{
	    let result: A<T> = a.bitor(b);
	    assert_eq!(result.value, 0x7f as T);
	}
	{
	    let result: A<T> = a | b;
	    assert_eq!(result.value, 0x7f as T);
	}
    }

    #[test]
    fn bitor_test2() {
	let a: bool = false;
	let b: bool = true;
	{
	    let result: bool = a.bitor(b);
	    assert_eq!(result, true);
	}

	let c: &bool = &false;
	{
	    let result: bool = c | a;
	    assert_eq!(result, false);
	}
	{
	    let result: bool = b | c;
	    assert_eq!(result, true);
	}
	{
	    let result: bool = c | c;
	    assert_eq!(result, false);
	}
    }

    #[test]
    fn bitor_test3() {
	bitor_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 };
    }
}
