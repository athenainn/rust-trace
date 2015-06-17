#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! fmt_refs {
    //     ($($tr:ident),*) => {
    //         $(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl<'a, T: ?Sized + $tr> $tr for &'a T {
    //             fn fmt(&self, f: &mut Formatter) -> Result { $tr::fmt(&**self, f) }
    //         }
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl<'a, T: ?Sized + $tr> $tr for &'a mut T {
    //             fn fmt(&self, f: &mut Formatter) -> Result { $tr::fmt(&**self, f) }
    //         }
    //         )*
    //     }
    // }

    // fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<'a, T> Pointer for &'a mut T {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         // FIXME(#23542) Replace with type ascription.
    //         #![allow(trivial_casts)]
    //         Pointer::fmt(&(&**self as *const T), f)
    //     }
    // }

    type T = i32;

    #[test]
    fn fmt_test1() {
	let mut value: T = 68;
	let value_refmut: &mut T = &mut value;
	let output: String = format!("{:?}", value_refmut);

	assert_eq!(output, "68".to_string());
    }

    #[test]
    fn fmt_test2() {
	let mut value: T = 68;
	let value_refmut: &mut T = &mut value;
	let output: String = format!("{}", value_refmut);

	assert_eq!(output, "68".to_string());
    }

    #[test]
    fn fmt_test3() {
	let mut value: T = 68;
	let value_refmut: &mut T = &mut value;
	let output: String = format!("{:o}", value_refmut);

	assert_eq!(output, "104".to_string());
    }

    #[test]
    fn fmt_test4() {
	let mut value: T = 68;
	let value_refmut: &mut T = &mut value;
	let output: String = format!("{:b}", value_refmut);

	assert_eq!(output, "1000100".to_string());
    }

    #[test]
    fn fmt_test5() {
	let mut value: T = 500;
	let value_refmut: &mut T = &mut value;
	let output: String = format!("{:x}", value_refmut);

	assert_eq!(output, "1f4".to_string());
    }

    #[test]
    fn fmt_test6() {
	let mut value: T = 500;
	let value_refmut: &mut T = &mut value;
	let output: String = format!("{:X}", value_refmut);

	assert_eq!(output, "1F4".to_string());
    }

    #[test]
    fn fmt_test7() {
	let mut value: f32 = 3.141592654;
	let value_refmut: &mut f32 = &mut value;
	let output: String = format!("{:e}", value_refmut);

	assert_eq!(output, "3.1415927e0".to_string());
    }

    #[test]
    fn fmt_test8() {
	let mut value: f32 = 3.141592654;
	let value_refmut: &mut f32 = &mut value;
	let output: String = format!("{:E}", value_refmut);

	assert_eq!(output, "3.1415927E0".to_string());
    }

    #[test]
    fn fmt_test9() {
	let mut value: T = 68;
	let value_refmut: &mut T = &mut value;
	let _: String = format!("{:p}", value_refmut); // "0x112200c7c"
	let _: String = format!("{:#p}", value_refmut); // "0x000000010da06c7c"
	let _: String = format!("{:#25p}", value_refmut); // "0x00000000000000114206c7c"
    }
}
