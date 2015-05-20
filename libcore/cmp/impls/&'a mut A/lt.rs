#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     impl<'a, 'b, A: ?Sized, B: ?Sized> PartialOrd<&'b mut B> for &'a mut A where A: PartialOrd<B> {
    //         #[inline]
    //         fn partial_cmp(&self, other: &&'b mut B) -> Option<Ordering> {
    //             PartialOrd::partial_cmp(*self, *other)
    //         }
    //         #[inline]
    //         fn lt(&self, other: &&'b mut B) -> bool { PartialOrd::lt(*self, *other) }
    //         #[inline]
    //         fn le(&self, other: &&'b mut B) -> bool { PartialOrd::le(*self, *other) }
    //         #[inline]
    //         fn ge(&self, other: &&'b mut B) -> bool { PartialOrd::ge(*self, *other) }
    //         #[inline]
    //         fn gt(&self, other: &&'b mut B) -> bool { PartialOrd::gt(*self, *other) }
    //     }

    macro_rules! lt_test {
	($($t:ty)*) => ($({
	    let v1: &mut $t = &mut (68 as $t);
	    {
		let result: bool = v1.lt(&v1);
		assert_eq!(result, false);
	    }

	    let v2: &mut $t = &mut (100 as $t);
	    {
		let result: bool = v1.lt(&v2);
		assert_eq!(result, true);
	    }

	    {
		let result: bool = v2.lt(&v1);
		assert_eq!(result, false);
	    }
	})*)
    }

    #[test]
    fn lt_test1() {
	lt_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
