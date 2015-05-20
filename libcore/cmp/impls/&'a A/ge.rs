#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
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

    macro_rules! ge_test {
	($($t:ty)*) => ($({
	    let v1: &$t = &(68 as $t);
	    {
		let resuge: bool = v1.ge(&v1);
		assert_eq!(resuge, true);
	    }

	    let v2: &$t = &(100 as $t);
	    {
		let resuge: bool = v1.ge(&v2);
		assert_eq!(resuge, false);
	    }

	    {
		let resuge: bool = v2.ge(&v1);
		assert_eq!(resuge, true);
	    }
	})*)
    }

    #[test]
    fn ge_test1() {
	ge_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
