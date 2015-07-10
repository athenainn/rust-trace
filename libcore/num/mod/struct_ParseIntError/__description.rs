#![feature(core, int_error_internals)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::ParseIntError;

    use core::str::FromStr;

    // #[derive(Debug, Clone, PartialEq)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub struct ParseIntError { kind: IntErrorKind }

    // #[derive(Debug, Clone, PartialEq)]
    // enum IntErrorKind {
    //     Empty,
    //     InvalidDigit,
    //     Overflow,
    //     Underflow,
    // }

    // impl ParseIntError {
    //     #[unstable(feature = "int_error_internals",
    //                reason = "available through Error trait and this method should \
    //                          not be exposed publicly")]
    //     #[doc(hidden)]
    //     pub fn __description(&self) -> &str {
    //         match self.kind {
    //             IntErrorKind::Empty => "cannot parse integer from empty string",
    //             IntErrorKind::InvalidDigit => "invalid digit found in string",
    //             IntErrorKind::Overflow => "number too large to fit in target type",
    //             IntErrorKind::Underflow => "number too small to fit in target type",
    //         }
    //     }
    // }

    // macro_rules! from_str_radix_int_impl {
    //     ($($t:ty)*) => {$(
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         #[allow(deprecated)]
    //         impl FromStr for $t {
    //             type Err = ParseIntError;
    //             fn from_str(src: &str) -> Result<Self, ParseIntError> {
    //                 from_str_radix(src, 10)
    //             }
    //         }
    //     )*}
    // }

    // from_str_radix_int_impl! { isize i8 i16 i32 i64 usize u8 u16 u32 u64 }

    type T = i32;

    #[test]
    fn __description_test1() {
	let src: &'static str = "";
	let from_str: Result<T, ParseIntError> = T::from_str(src);

	let parseinterror: ParseIntError = from_str.unwrap_err();
	let description: &str = parseinterror.__description();
	assert_eq!(description, "cannot parse integer from empty string");
    }

    #[test]
    fn __description_test2() {
	let src: &'static str = "Hello, World!";
	let from_str: Result<T, ParseIntError> = T::from_str(src);

	let parseinterror: ParseIntError = from_str.unwrap_err();
	let description: &str = parseinterror.__description();
	assert_eq!(description, "invalid digit found in string");
    }

    #[test]
    fn __description_test3() {
	let src: &'static str = "99999999999999999999";
	let from_str: Result<T, ParseIntError> = T::from_str(src);

	let parseinterror: ParseIntError = from_str.unwrap_err();
	let description: &str = parseinterror.__description();
	assert_eq!(description, "number too large to fit in target type");
    }

    #[test]
    fn __description_test4() {
	let src: &'static str = "-99999999999999999999";
	let from_str: Result<T, ParseIntError> = T::from_str(src);

	let parseinterror: ParseIntError = from_str.unwrap_err();
	let description: &str = parseinterror.__description();
	assert_eq!(description, "number too small to fit in target type");
    }
}
