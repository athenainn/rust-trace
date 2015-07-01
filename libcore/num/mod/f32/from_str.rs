#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::ParseFloatError;
    use core::num::Float;

    use core::str::FromStr;

    // pub struct ParseFloatError {
    //     #[doc(hidden)]
    //     pub __kind: FloatErrorKind
    // }

    // #[derive(Debug, Clone, PartialEq)]
    // pub enum FloatErrorKind {
    //     Empty,
    //     Invalid,
    // }

    // impl ParseFloatError {
    //     #[doc(hidden)]
    //     pub fn __description(&self) -> &str {
    //         match self.__kind {
    //             FloatErrorKind::Empty => "cannot parse float from empty string",
    //             FloatErrorKind::Invalid => "invalid float literal",
    //         }
    //     }
    // }

    // macro_rules! from_str_float_impl {
    //     ($t:ty) => {
    //         #[stable(feature = "rust1", since = "1.0.0")]
    //         impl FromStr for $t {
    //             type Err = ParseFloatError;
    //
    //             /// Converts a string in base 10 to a float.
    //             /// Accepts an optional decimal exponent.
    //             ///
    //             /// This function accepts strings such as
    //             ///
    //             /// * '3.14'
    //             /// * '-3.14'
    //             /// * '2.5E10', or equivalently, '2.5e10'
    //             /// * '2.5E-10'
    //             /// * '.' (understood as 0)
    //             /// * '5.'
    //             /// * '.5', or, equivalently,  '0.5'
    //             /// * 'inf', '-inf', 'NaN'
    //             ///
    //             /// Leading and trailing whitespace represent an error.
    //             ///
    //             /// # Arguments
    //             ///
    //             /// * src - A string
    //             ///
    //             /// # Return value
    //             ///
    //             /// `Err(ParseFloatError)` if the string did not represent a valid
    //             /// number.  Otherwise, `Ok(n)` where `n` is the floating-point
    //             /// number represented by `src`.
    //             #[inline]
    //             #[allow(deprecated)]
    //             fn from_str(src: &str) -> Result<Self, ParseFloatError> {
    //                 Self::from_str_radix(src, 10)
    //             }
    //         }
    //     }
    // }

    // from_str_float_impl!(f32);

    #[test]
    fn from_str_test1() {
	let src: &'static str = "3.14";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(3.1399999));
    }

    #[test]
    fn from_str_test2() {
	let src: &'static str = "-3.14";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(-3.1399999));
    }

    #[test]
    fn from_str_test3() {
	let src: &'static str = "2.5E10";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(25000000000f32));
    }

    #[test]
    fn from_str_test4() {
	let src: &'static str = "2.5e10";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(25000000000f32));
    }

    #[test]
    fn from_str_test5() {
	let src: &'static str = "2.5E-10";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(0.00000000025));
    }

    #[test]
    fn from_str_test6() {
	let src: &'static str = "2.5e-10";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(0.00000000025));
    }

    #[test]
    fn from_str_test7() {
	let src: &'static str = ".";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(0f32));
    }

    #[test]
    fn from_str_test8() {
	let src: &'static str = "5.";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(5f32));
    }

    #[test]
    fn from_str_test9() {
	let src: &'static str = ".5";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(0.5f32));
    }

    #[test]
    fn from_str_test10() {
	let src: &'static str = "0.5";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(0.5f32));
    }

    #[test]
    fn from_str_test11() {
	let src: &'static str = "inf";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(f32::infinity()));
    }

    #[test]
    fn from_str_test12() {
	let src: &'static str = "-inf";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result, Ok::<f32, ParseFloatError>(f32::neg_infinity()));
    }

    #[test]
    fn from_str_test13() {
	let src: &'static str = "NaN";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	match result {
	    Ok(v) => assert_eq!(v.is_nan(), true),
	    Err(_) => assert!(false)
	}
    }

    #[test]
    fn from_str_test14() {
	let src: &'static str = "";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result.unwrap_err().__description(), "cannot parse float from empty string");
    }

    #[test]
    fn from_str_test15() {
	let src: &'static str = "Hello, World!";
	let result: Result<f32, ParseFloatError> = f32::from_str(src);

	assert_eq!(result.unwrap_err().__description(), "invalid float literal");
    }
}
