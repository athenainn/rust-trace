#![feature(core, float_error_internals)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::ParseFloatError;
    use core::num::FloatErrorKind::{Empty, Invalid};

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
    //             /// * '+3.14', equivalent to '3.14'
    //             /// * '-3.14'
    //             /// * '2.5E10', or equivalently, '2.5e10'
    //             /// * '2.5E-10'
    //             /// * '.' (understood as 0)
    //             /// * '5.'
    //             /// * '.5', or, equivalently,  '0.5'
    //             /// * '+inf', 'inf', '-inf', 'NaN'
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
    // from_str_float_impl!(f64);

    type T = f32;

    #[test]
    fn __description_test1() {
	let src: &'static str = "";
	let from_str: Result<T, ParseFloatError> = T::from_str(src);

	let parsefloaterror: ParseFloatError = from_str.unwrap_err();
	assert_eq!(parsefloaterror.__kind, Empty);

	let description: &str = parsefloaterror.__description();
	assert_eq!(description, "cannot parse float from empty string");
    }

    #[test]
    fn __description_test2() {
	let src: &'static str = "Hello, Wolrd!";
	let from_str: Result<T, ParseFloatError> = T::from_str(src);

	let parsefloaterror: ParseFloatError = from_str.unwrap_err();
	assert_eq!(parsefloaterror.__kind, Invalid);

	let description: &str = parsefloaterror.__description();
	assert_eq!(description, "invalid float literal");
    }
}
