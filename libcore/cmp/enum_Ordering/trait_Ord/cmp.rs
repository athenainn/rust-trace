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

    // impl Ord for Ordering {
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn cmp(&self, other: &Ordering) -> Ordering {
    //         (*self as i32).cmp(&(*other as i32))
    //     }
    // }

    #[test]
    fn cmp_test1() {
	let x: Ordering = Less;
	let y: Ordering = Less;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Equal);
    }

    #[test]
    fn cmp_test2() {
	let x: Ordering = Equal;
	let y: Ordering = Less;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Greater);
    }

    #[test]
    fn cmp_test3() {
	let x: Ordering = Greater;
	let y: Ordering = Less;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Greater);
    }

    #[test]
    fn cmp_test4() {
	let x: Ordering = Less;
	let y: Ordering = Equal;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Less);
    }

    #[test]
    fn cmp_test5() {
	let x: Ordering = Equal;
	let y: Ordering = Equal;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Equal);
    }

    #[test]
    fn cmp_test6() {
	let x: Ordering = Greater;
	let y: Ordering = Equal;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Greater);
    }

    #[test]
    fn cmp_test7() {
	let x: Ordering = Less;
	let y: Ordering = Greater;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Less);
    }

    #[test]
    fn cmp_test8() {
	let x: Ordering = Equal;
	let y: Ordering = Greater;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Less);
    }

    #[test]
    fn cmp_test9() {
	let x: Ordering = Greater;
	let y: Ordering = Greater;
	let result: Ordering = x.cmp(&y);

	assert_eq!(result, Equal);
    }
}
