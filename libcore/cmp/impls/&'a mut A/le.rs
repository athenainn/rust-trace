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

    macro_rules! le_test {
	($($t:ty)*) => ($({
	    let v1: &mut $t = &mut (68 as $t);
	    {
		let resule: bool = v1.le(&v1);
		assert_eq!(resule, true);
	    }

	    let v2: &mut $t = &mut (100 as $t);
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
