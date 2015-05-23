#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::BitAnd;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl BitAnd for A<T> {
	type Output = A<T>;

	fn bitand(self, rhs: A<T>) -> Self::Output {
	    A { value: self.value & rhs.value }
	}
    }

    // pub trait BitAnd<RHS=Self> {
    //     /// The resulting type after applying the `&` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the `&` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn bitand(self, rhs: RHS) -> Self::Output;
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

    // macro_rules! bitand_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl BitAnd for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn bitand(self, rhs: $t) -> $t { self & rhs }
    //         }
    //
    //         forward_ref_binop! { impl BitAnd, bitand for $t, $t }
    //     )*)
    // }

    // bitand_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

    type T = i32;

    macro_rules! bitand_test {
	($($t:ty)*) => ($({
	    let a: $t = 0x7f as $t;
	    let b: $t = 0xf as $t;
	    {
		let result: $t = a.bitand(b);
		assert_eq!(result, 0xf as $t);
	    }
	    {
		let result: $t = a & b;
		assert_eq!(result, 0xf as $t);
	    }

	    let c: &$t = &(0xf as $t);
	    {
		let result: $t = c & a;
		assert_eq!(result, 0xf as $t);
	    }
	    {
		let result: $t = a & c;
		assert_eq!(result, 0xf as $t);
	    }
	    {
		let result: $t = c & c;
		assert_eq!(result, 0xf as $t);
	    }
	})*)
    }

    #[test]
    fn bitand_test1() {
	let a: A<T> = A { value: 0x7f };
	let b: A<T> = A { value: 0xf };
	{
	    let result: A<T> = a.bitand(b);
	    assert_eq!(result.value, 0xf as T);
	}
	{
	    let result: A<T> = a & b;
	    assert_eq!(result.value, 0xf as T);
	}
    }

    #[test]
    fn bitand_test2() {
	let a: bool = false;
	let b: bool = true;
	{
	    let result: bool = a.bitand(b);
	    assert_eq!(result, false);
	}

	let c: &bool = &true;
	{
	    let result: bool = c & a;
	    assert_eq!(result, false);
	}
	{
	    let result: bool = b & c;
	    assert_eq!(result, true);
	}
	{
	    let result: bool = c & c;
	    assert_eq!(result, true);
	}
    }

    #[test]
    fn bitand_test3() {
	bitand_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 };
    }
}
