#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cmp::Ordering::{self, Less, Equal, Greater};

    //     impl<'a, 'b, A: ?Sized, B: ?Sized> PartialOrd<&'b B> for &'a A where A: PartialOrd<B> {
    //         #[inline]
    //         fn partial_cmp(&self, other: &&'b B) -> Option<Ordering> {
    //             PartialOrd::partial_cmp(*self, *other)
    //         }
    //         #[inline]
    //         fn lt(&self, other: & &'b B) -> bool { PartialOrd::lt(*self, *other) }
    //         #[inline]
    //         fn le(&self, other: & &'b B) -> bool { PartialOrd::le(*self, *other) }
    //         #[inline]
    //         fn ge(&self, other: & &'b B) -> bool { PartialOrd::ge(*self, *other) }
    //         #[inline]
    //         fn gt(&self, other: & &'b B) -> bool { PartialOrd::gt(*self, *other) }
    //     }

    macro_rules! partial_cmp_test {
	($($t:ty)*) => ($({
	    let v1: &$t = &(68 as $t);
	    {
		let result: Option<Ordering> = v1.partial_cmp(&v1);
		match result {
		    Some(v) => assert_eq!(v, Equal),
		    None => assert!(false)
		}
	    }

	    let v2: &$t = &(100 as $t);
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
