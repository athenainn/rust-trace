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

    macro_rules! le_test {
	($($t:ty)*) => ($({
	    let v1: $t = 68 as $t;
	    {
		let resule: bool = v1.le(&v1);
		assert_eq!(resule, true);
	    }

	    let v2: $t = 100 as $t;
	    {
		let resule: bool = v1.le(&v2);
		assert_eq!(resule, true);
	    }

	    {
		let resule: bool = v2.le(&v1);
		assert_eq!(resule, false);
	    }
	})*)
    }

    #[test]
    fn le_test1() {
	le_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
