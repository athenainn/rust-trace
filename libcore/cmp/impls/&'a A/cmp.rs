#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     impl<'a, A: ?Sized> Ord for &'a A where A: Ord {
    //         #[inline]
    //         fn cmp(&self, other: & &'a A) -> Ordering { Ord::cmp(*self, *other) }
    //     }

    macro_rules! cmp_test {
	($($t:ty)*) => ($({
	    let v1: &$t = &(68 as $t);
	    {
		let result: Ordering = v1.cmp(&v1);
		assert_eq!(result, Equal);
	    }

	    let v2: &$t = &(100 as $t);
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
