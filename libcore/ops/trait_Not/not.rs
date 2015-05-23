#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Not;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Not for A<T> {
	type Output = A<T>;

	fn not(self) -> Self::Output {
	    A { value: !self.value }
	}
    }

    // pub trait Not {
    //     /// The resulting type after applying the `!` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the unary `!` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn not(self) -> Self::Output;
    // }

    // macro_rules! forward_ref_unop {
    //     (impl $imp:ident, $method:ident for $t:ty) => {
    //         #[unstable(feature = "core",
    //                    reason = "recently added, waiting for dust to settle")]
    //         impl<'a> $imp for &'a $t {
    //             type Output = <$t as $imp>::Output;
    //
    //             #[inline]
    //             fn $method(self) -> <$t as $imp>::Output {
    //                 $imp::$method(*self)
    //             }
    //         }
    //     }
    // }

    // macro_rules! not_impl {
    //     ($($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl Not for $t {
    //             type Output = $t;
    //
    //             #[inline]
    //             fn not(self) -> $t { !self }
    //         }
    //
    //         forward_ref_unop! { impl Not, not for $t }
    //     )*)
    // }

    // not_impl! { bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

    type T = i32;

    macro_rules! not_test {
	($($t:ty)*) => ($({
	    let a: $t = 68 as $t;
	    {
		let result: $t = a.not();
		assert_eq!(result, !68 as $t);
	    }
	    {
		let result: $t = !a;
		assert_eq!(result, !68 as $t);
	    }

	    let b: &$t = &(68 as $t);
	    {
		let result: $t = b.not();
		assert_eq!(result, !68 as $t);
	    }
	    {
		let result: $t = !b;
		assert_eq!(result, !68 as $t);
	    }
	})*)
    }

    #[test]
    fn not_test1() {
	let a: A<T> = A { value: 68 };
	{
	    let result: A<T> = a.not();
	    assert_eq!(result.value, !68 as T);
	}
	{
	    let result: A<T> = !a;
	    assert_eq!(result.value, !68 as T);
	}
    }

    #[test]
    fn not_test2() {
	let a: bool = true;
	{
	    let result: bool = a.not();
	    assert_eq!(result, false);
	}

	let b: &bool = &true;
	{
	    let result: bool = !b;
	    assert_eq!(result, false);
	}
    }

    #[test]
    fn not_test3() {
	not_test! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 };
    }
}
