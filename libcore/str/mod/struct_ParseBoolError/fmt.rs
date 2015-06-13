#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use core::str::ParseBoolError;

    // pub trait FromStr {
    //     /// The associated error which can be returned from parsing.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     type Err;
    //
    //     /// Parses a string `s` to return a value of this type.
    //     ///
    //     /// If parsing succeeds, return the value inside `Ok`, otherwise
    //     /// when the string is ill-formatted return an error specific to the
    //     /// inside `Err`. The error type is specific to implementation of the trait.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn from_str(s: &str) -> Result<Self, Self::Err>;
    // }

    // impl FromStr for bool {
    //     type Err = ParseBoolError;
    //
    //     /// Parse a `bool` from a string.
    //     ///
    //     /// Yields a `Result<bool, ParseBoolError>`, because `s` may or may not
    //     /// actually be parseable.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::str::FromStr;
    //     ///
    //     /// assert_eq!(FromStr::from_str("true"), Ok(true));
    //     /// assert_eq!(FromStr::from_str("false"), Ok(false));
    //     /// assert!(<bool as FromStr>::from_str("not even a boolean").is_err());
    //     /// ```
    //     ///
    //     /// Note, in many cases, the `.parse()` method on `str` is more proper.
    //     ///
    //     /// ```
    //     /// assert_eq!("true".parse(), Ok(true));
    //     /// assert_eq!("false".parse(), Ok(false));
    //     /// assert!("not even a boolean".parse::<bool>().is_err());
    //     /// ```
    //     #[inline]
    //     fn from_str(s: &str) -> Result<bool, ParseBoolError> {
    //         match s {
    //             "true"  => Ok(true),
    //             "false" => Ok(false),
    //             _       => Err(ParseBoolError { _priv: () }),
    //         }
    //     }
    // }

    // #[derive(Debug, Clone, PartialEq)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub struct ParseBoolError { _priv: () }

    #[test]
    fn fmt_test1() {
	let s: &str = "hello";
	let from_str: Result<bool, ParseBoolError> = bool::from_str(s);
	let unwrap_err: ParseBoolError = from_str.unwrap_err();
	let result: String = format!("{}", unwrap_err);

	assert_eq!(result, "provided string was not `true` or `false`");
    }

    #[test]
    fn fmt_test2() {
	let s: &str = "hello";
	let from_str: Result<bool, <bool as FromStr>::Err> = bool::from_str(s);
	let unwrap_err: ParseBoolError = from_str.unwrap_err();
	let result: String = format!("{}", unwrap_err);

	assert_eq!(result, "provided string was not `true` or `false`");
    }
}
