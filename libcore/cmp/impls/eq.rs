#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     macro_rules! partial_eq_impl {
    //         ($($t:ty)*) => ($(
    //             #[stable(feature = "rust1", since = "1.0.0")]
    //             impl PartialEq for $t {
    //                 #[inline]
    //                 fn eq(&self, other: &$t) -> bool { (*self) == (*other) }
    //                 #[inline]
    //                 fn ne(&self, other: &$t) -> bool { (*self) != (*other) }
    //             }
    //         )*)
    //     }

    //     partial_eq_impl! {
    //         bool char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64
    //     }

    macro_rules! eq_test {
	($($t:ty)*) => ($({
	    let v1: $t = 68 as $t;
	    {
		let result: bool = v1.eq(&v1);
		assert_eq!(result, true);
	    }

	    let v2: $t = 100 as $t;
	    {
		let result: bool = v1.eq(&v2);
		assert_eq!(result, false);
	    }
	})*)
    }

    #[test]
    fn eq_test1() {
	let v1: bool = false;
	{
	    let result: bool = v1.eq(&v1);
	    assert_eq!(result, true);
	}

	let v2: bool = true;
	{
	    let result: bool = v1.eq(&v2);
	    assert_eq!(result, false);
	}
    }

    #[test]
    fn eq_test2() {
	eq_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
