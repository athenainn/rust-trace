#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Equal};

    //     impl PartialOrd for () {
    //         #[inline]
    //         fn partial_cmp(&self, _: &()) -> Option<Ordering> {
    //             Some(Equal)
    //         }
    //     }

    #[test]
    fn partial_cmp_test1() {
	let v1: () = ();
	let v2: () = ();
	let result: Option<Ordering> = v1.partial_cmp(&v2);

	match result {
	    Some(v) => assert_eq!(v, Equal),
	    None => assert!(false)
	}
    }
}
