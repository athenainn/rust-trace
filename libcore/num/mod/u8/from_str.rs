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

    #[test]
    fn from_str_test1() {
	let src: &'static str = "68";
	let result: Result<u8, ParseIntError> = u8::from_str(src);

	assert_eq!(result, Ok::<u8, ParseIntError>(68));
    }

    #[test]
    fn from_str_test2() {
	let src: &'static str = "";
	let result: Result<u8, ParseIntError> = u8::from_str(src);

	assert_eq!(result.unwrap_err().__description(), "cannot parse integer from empty string");
    }

    #[test]
    fn from_str_test3() {
	let src: &'static str = "3.14";
	let result: Result<u8, ParseIntError> = u8::from_str(src);

	assert_eq!(result.unwrap_err().__description(), "invalid digit found in string");
    }

    #[test]
    fn from_str_test4() {
	let src: &'static str = "256";
	let result: Result<u8, ParseIntError> = u8::from_str(src);

	assert_eq!(result.unwrap_err().__description(), "number too large to fit in target type");
    }
}
