#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    //     impl<'a, 'b, A: ?Sized, B: ?Sized> PartialEq<&'b B> for &'a A where A: PartialEq<B> {
    //         #[inline]
    //         fn eq(&self, other: & &'b B) -> bool { PartialEq::eq(*self, *other) }
    //         #[inline]
    //         fn ne(&self, other: & &'b B) -> bool { PartialEq::ne(*self, *other) }
    //     }

    //     impl<'a, 'b, A: ?Sized, B: ?Sized> PartialEq<&'b mut B> for &'a A where A: PartialEq<B> {
    //         #[inline]
    //         fn eq(&self, other: &&'b mut B) -> bool { PartialEq::eq(*self, *other) }
    //         #[inline]
    //         fn ne(&self, other: &&'b mut B) -> bool { PartialEq::ne(*self, *other) }
    //     }

    macro_rules! eq_test {
	($($t:ty)*) => ($({
	    let v1: &$t = &(68 as $t);
	    {
		let result: bool = v1.eq(&v1);
		assert_eq!(result, true);
	    }

	    let v2: &$t = &(100 as $t);
	    {
		let result: bool = v1.eq(&v2);
		assert_eq!(result, false);
	    }

	    let v3: &mut $t = &mut (68 as $t);
	    {
		let result: bool = v1.eq(&v3);
		assert_eq!(result, true);
	    }

	    let v4: &$t = &mut (100 as $t);
	    {
		let result: bool = v1.eq(&v4);
		assert_eq!(result, false);
	    }
	})*)
    }

    #[test]
    fn eq_test1() {
	let v1: &bool = &false;
	{
	    let result: bool = v1.eq(&v1);
	    assert_eq!(result, true);
	}

	let v2: &bool = &true;
	{
	    let result: bool = v1.eq(&v2);
	    assert_eq!(result, false);
	}

	let v3: &mut bool = &mut false;
	{
	    let result: bool = v1.eq(&v3);
	    assert_eq!(result, true);
	}

	let v4: &mut bool = &mut true;
	{
	    let result: bool = v1.eq(&v4);
	    assert_eq!(result, false);
	}
    }

    #[test]
    fn eq_test2() {
	eq_test! { char usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 };
    }
}
