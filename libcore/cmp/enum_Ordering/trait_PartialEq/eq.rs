#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    // #[derive(Clone, Copy, PartialEq, Debug)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Ordering {
    //     /// An ordering where a compared value is less [than another].
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Less = -1,
    //     /// An ordering where a compared value is equal [to another].
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Equal = 0,
    //     /// An ordering where a compared value is greater [than another].
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Greater = 1,
    // }

    // pub trait Eq: PartialEq<Self> {
    //     // FIXME #13101: this method is used solely by #[deriving] to
    //     // assert that every component of a type implements #[deriving]
    //     // itself, the current deriving infrastructure means doing this
    //     // assertion without using a method on this trait is nearly
    //     // impossible.
    //     //
    //     // This should never be implemented by hand.
    //     #[doc(hidden)]
    //     #[inline(always)]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn assert_receiver_is_total_eq(&self) {}
    // }

    // impl Eq for Ordering {}

    #[test]
    fn eq_test1() {
	let x: Ordering = Less;
	let y: Ordering = Less;
	let result: bool = x.eq(&y);

	assert_eq!(result, true);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test2() {
	let x: Ordering = Equal;
	let y: Ordering = Less;
	let result: bool = x.eq(&y);

	assert_eq!(result, false);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test3() {
	let x: Ordering = Greater;
	let y: Ordering = Less;
	let result: bool = x.eq(&y);

	assert_eq!(result, false);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test4() {
	let x: Ordering = Less;
	let y: Ordering = Equal;
	let result: bool = x.eq(&y);

	assert_eq!(result, false);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test5() {
	let x: Ordering = Equal;
	let y: Ordering = Equal;
	let result: bool = x.eq(&y);

	assert_eq!(result, true);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test6() {
	let x: Ordering = Greater;
	let y: Ordering = Equal;
	let result: bool = x.eq(&y);

	assert_eq!(result, false);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test7() {
	let x: Ordering = Less;
	let y: Ordering = Greater;
	let result: bool = x.eq(&y);

	assert_eq!(result, false);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test8() {
	let x: Ordering = Equal;
	let y: Ordering = Greater;
	let result: bool = x.eq(&y);

	assert_eq!(result, false);
	assert_eq!(result, x == y);
    }

    #[test]
    fn eq_test9() {
	let x: Ordering = Greater;
	let y: Ordering = Greater;
	let result: bool = x.eq(&y);

	assert_eq!(result, true);
	assert_eq!(result, x == y);
    }
}
