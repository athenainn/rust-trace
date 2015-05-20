#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     macro_rules! partial_ord_impl {
    //         ($($t:ty)*) => ($(
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl PartialOrd for $t {
    //                 #[inline]
    //                 fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
    //                     match (self <= other, self >= other) {
    //                         (false, false) => None,
    //                         (false, true) => Some(Greater),
    //                         (true, false) => Some(Less),
    //                         (true, true) => Some(Equal),
    //                     }
    //                 }
    //                 #[inline]
    //                 fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
    //                 #[inline]
    //                 fn le(&self, other: &$t) -> bool { (*self) <= (*other) }
    //                 #[inline]
    //                 fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
    //                 #[inline]
    //                 fn gt(&self, other: &$t) -> bool { (*self) > (*other) }
    //             }
    //         )*)
    //     }

    // partial_ord_impl! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }

    macro_rules! partial_cmp_test {
	($($t:ty)*) => ($({
	    let v1: $t = 68 as $t;
	    {
		let result: Option<Ordering> = v1.partial_cmp(&v1);
		match result {
		    Some(v) => assert_eq!(v, Equal),
		    None => assert!(false)
		}
	    }

	    let v2: $t = 100 as $t;
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
	})*)
    }

    #[test]
    fn partial_cmp_test1() {
	partial_cmp_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
