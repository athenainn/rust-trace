#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Equal};

    //     impl Ord for () {
    //         #[inline]
    //         fn cmp(&self, _other: &()) -> Ordering { Equal }
    //     }

    #[test]
    fn cmp_test1() {
	let v1: () = ();
	let v2: () = ();
	let result: Ordering = v1.cmp(&v2);

	assert_eq!(result, Equal);
    }
}
