#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::ops::Neg;

    #[derive(Copy, Clone)]
    struct A<T> {
	value: T
    }

    impl Neg for A<T> {
	type Output = A<T>;

	fn neg(self) -> Self::Output {
	    A { value: -self.value }
	}
    }

    // pub trait Neg {
    //     /// The resulting type after applying the `-` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Output;
    //
    //     /// The method for the unary `-` operator
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn neg(self) -> Self::Output;
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

    // macro_rules! neg_impl_core {
    //     ($id:ident => $body:expr, $($t:ty)*) => ($(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[allow(unsigned_negation)]
    //         impl Neg for $t {
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             type Output = $t;
    //
    //             #[inline]
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             fn neg(self) -> $t { let $id = self; $body }
    //         }
    //
    //         forward_ref_unop! { impl Neg, neg for $t }
    //     )*)
    // }

    // macro_rules! neg_impl_numeric {
    //     ($($t:ty)*) => { neg_impl_core!{ x => -x, $($t)*} }
    // }

    // neg_impl_numeric! { isize i8 i16 i32 i64 f32 f64 }

    type T = i32;

    macro_rules! neg_test {
	($($t:ty)*) => ($({
	    let a: $t = 68 as $t;
	    {
		let result: $t = a.neg();
		assert_eq!(result, -68 as $t);
	    }
	    {
		let result: $t = -a;
		assert_eq!(result, -68 as $t);
	    }

	    let b: &$t = &(68 as $t);
	    {
		let result: $t = b.neg();
		assert_eq!(result, -68 as $t);
	    }
	    {
		let result: $t = -b;
		assert_eq!(result, -68 as $t);
	    }
	})*)
    }

    #[test]
    fn neg_test1() {
	let a: A<T> = A { value: 68 };
	{
	    let b: A<T> = a.neg();
	    assert_eq!(b.value, -68);
	}
	{
	    let b: A<T> = -a;
	    assert_eq!(b.value, -68);
	}
    }

    #[test]
    fn neg_test2() {
	neg_test! { isize i8 i16 i32 i64 f32 f64 };
    }
}
