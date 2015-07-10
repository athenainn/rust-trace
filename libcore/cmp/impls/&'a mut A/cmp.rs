#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     impl<'a, A: ?Sized> Ord for &'a mut A where A: Ord {
    //         #[inline]
    //         fn cmp(&self, other: &&'a mut A) -> Ordering { Ord::cmp(*self, *other) }
    //     }

    macro_rules! cmp_test {
	($($t:ty)*) => ($({
	    let v1: &mut $t = &mut (68 as $t);
	    {
		let result: Ordering = v1.cmp(&v1);
		assert_eq!(result, Equal);
	    }

	    let v2: &mut $t = &mut (100 as $t);
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
