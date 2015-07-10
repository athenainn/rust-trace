#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     macro_rules! ord_impl {
    //         ($($t:ty)*) => ($(
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl Ord for $t {
    //                 #[inline]
    //                 fn cmp(&self, other: &$t) -> Ordering {
    //                     if *self < *other { Less }
    //                     else if *self > *other { Greater }
    //                     else { Equal }
    //                 }
    //             }
    //         )*)
    //     }

    //     ord_impl! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 }

    macro_rules! cmp_test {
	($($t:ty)*) => ($({
	    let v1: $t = 68 as $t;
	    {
		let result: Ordering = v1.cmp(&v1);
		assert_eq!(result, Equal);
	    }

	    let v2: $t = 100 as $t;
	    {
		let result: Ordering = v1.cmp(&v2);
		assert_eq!(result, Less);
	    }

	    {
		let result: Ordering = v2.cmp(&v1);
		assert_eq!(result, Greater);
	    }
	})*)
    }

    #[test]
    fn cmp_test1() {
	cmp_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 };
    }
}
