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

    // impl PartialOrd for Ordering {
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn partial_cmp(&self, other: &Ordering) -> Option<Ordering> {
    //         (*self as i32).partial_cmp(&(*other as i32))
    //     }
    // }

    #[test]
    fn partial_cmp_test1() {
	let x: Ordering = Less;
	let y: Ordering = Less;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Equal),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test2() {
	let x: Ordering = Less;
	let y: Ordering = Equal;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Less),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test3() {
	let x: Ordering = Less;
	let y: Ordering = Greater;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Less),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test4() {
	let x: Ordering = Equal;
	let y: Ordering = Less;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Greater),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test5() {
	let x: Ordering = Equal;
	let y: Ordering = Equal;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Equal),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test6() {
	let x: Ordering = Equal;
	let y: Ordering = Greater;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Less),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test7() {
	let x: Ordering = Greater;
	let y: Ordering = Less;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Greater),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test8() {
	let x: Ordering = Greater;
	let y: Ordering = Equal;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Greater),
	    None => assert!(false)
	}
    }

    #[test]
    fn partial_cmp_test9() {
	let x: Ordering = Greater;
	let y: Ordering = Greater;
	let result: Option<Ordering> = x.partial_cmp(&y);

	match result {
	    Some(v) => assert_eq!(v, Equal),
	    None => assert!(false)
	}
    }
}
