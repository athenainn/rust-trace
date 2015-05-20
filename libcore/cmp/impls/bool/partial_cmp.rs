#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     impl PartialOrd for bool {
    //         #[inline]
    //         fn partial_cmp(&self, other: &bool) -> Option<Ordering> {
    //             (*self as u8).partial_cmp(&(*other as u8))
    //         }
    //     }

    #[test]
    fn partial_cmp_test1() {
	let v1: bool = false;
	{
	    let result: Option<Ordering> = v1.partial_cmp(&v1);
	    match result {
		Some(v) => assert_eq!(v, Equal),
		None => assert!(false)
	    }
	}

	let v2: bool = true;
	{
	    let result: Option<Ordering> = v1.partial_cmp(&v2);
	    match result {
		Some(v) => assert_eq!(v, Less),
		None => assert!(false)
	    }
	}

	{
	    let result: Option<Ordering> = v2.partial_cmp(&v1);
	    match result {
		Some(v) => assert_eq!(v, Greater),
		None => assert!(false)
	    }
	}

    }
}
