#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
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

    macro_rules! gt_test {
	($($t:ty)*) => ($({
	    let v1: $t = 68 as $t;
	    {
		let resugt: bool = v1.gt(&v1);
		assert_eq!(resugt, false);
	    }

	    let v2: $t = 100 as $t;
	    {
		let resugt: bool = v1.gt(&v2);
		assert_eq!(resugt, false);
	    }

	    {
		let resugt: bool = v2.gt(&v1);
		assert_eq!(resugt, true);
	    }
	})*)
    }

    #[test]
    fn gt_test1() {
	gt_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
