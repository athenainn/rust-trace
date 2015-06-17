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

    // impl<'a, T> Pointer for &'a T {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         // FIXME(#23542) Replace with type ascription.
    //         #![allow(trivial_casts)]
    //         Pointer::fmt(&(*self as *const T), f)
    //     }
    // }

    type T = i32;

    #[test]
    fn fmt_test1() {
	let value: T = 68;
	let value_ref: &T = &value;
	let output: String = format!("{:?}", value_ref);

	assert_eq!(output, "68".to_string());
    }

    #[test]
    fn fmt_test2() {
	let value: T = 68;
	let value_ref: &T = &value;
	let output: String = format!("{}", value_ref);

	assert_eq!(output, "68".to_string());
    }

    #[test]
    fn fmt_test3() {
	let value: T = 68;
	let value_ref: &T = &value;
	let output: String = format!("{:o}", value_ref);

	assert_eq!(output, "104".to_string());
    }

    #[test]
    fn fmt_test4() {
	let value: T = 68;
	let value_ref: &T = &value;
	let output: String = format!("{:b}", value_ref);

	assert_eq!(output, "1000100".to_string());
    }

    #[test]
    fn fmt_test5() {
	let value: T = 500;
	let value_ref: &T = &value;
	let output: String = format!("{:x}", value_ref);

	assert_eq!(output, "1f4".to_string());
    }

    #[test]
    fn fmt_test6() {
	let value: T = 500;
	let value_ref: &T = &value;
	let output: String = format!("{:X}", value_ref);

	assert_eq!(output, "1F4".to_string());
    }

    #[test]
    fn fmt_test7() {
	let value: f32 = 3.141592654;
	let value_ref: &f32 = &value;
	let output: String = format!("{:e}", value_ref);

	assert_eq!(output, "3.1415927e0".to_string());
    }

    #[test]
    fn fmt_test8() {
	let value: f32 = 3.141592654;
	let value_ref: &f32 = &value;
	let output: String = format!("{:E}", value_ref);

	assert_eq!(output, "3.1415927E0".to_string());
    }

    #[test]
    fn fmt_test9() {
	let value: T = 68;
	let value_ref: &T = &value;
	let _: String = format!("{:p}", value_ref); // "0x105600c7c"
	let _: String = format!("{:#p}", value_ref); // "0x0000000108200c7c"
	let _: String = format!("{:#25p}", value_ref); // "0x00000000000000103206c7c"
    }
}
