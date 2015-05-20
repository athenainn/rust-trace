#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     impl Ord for bool {
    //         #[inline]
    //         fn cmp(&self, other: &bool) -> Ordering {
    //             (*self as u8).cmp(&(*other as u8))
    //         }
    //     }

    #[test]
    fn cmp_test1() {
	let v1: bool = false;
	{
	    let result: Ordering = v1.cmp(&v1);
	    assert_eq!(result, Equal);
	}

	let v2: bool = true;
	{
	    let result: Ordering = v1.cmp(&v2);
	    assert_eq!(result, Less);
	}

	{
	    let result: Ordering = v2.cmp(&v1);
	    assert_eq!(result, Greater);
	}

    }
}
