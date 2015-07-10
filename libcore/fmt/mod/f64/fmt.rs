#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // macro_rules! floating { ($ty:ident) => {
    //
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     impl Debug for $ty {
    //         fn fmt(&self, fmt: &mut Formatter) -> Result {
    //             float_to_decimal_common(fmt, self, true)
    //         }
    //     }
    //
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     impl Display for $ty {
    //         fn fmt(&self, fmt: &mut Formatter) -> Result {
    //             float_to_decimal_common(fmt, self, false)
    //         }
    //     }
    //
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     impl LowerExp for $ty {
    //         fn fmt(&self, fmt: &mut Formatter) -> Result {
    //             float_to_exponential_common(fmt, self, false)
    //         }
    //     }
    //
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     impl UpperExp for $ty {
    //         fn fmt(&self, fmt: &mut Formatter) -> Result {
    //             float_to_exponential_common(fmt, self, true)
    //         }
    //     }
    // } }
    // floating! { f32 }
    // floating! { f64 }

    type T = f64;

    #[test]
    fn fmt_test1() {
	let value: T = 3.141592654;

	let output: String = format!("{:?}", value);
	assert_eq!(output, "3.141592654".to_string());

	let output: String = format!("{:+?}", value);
	assert_eq!(output, "+3.141592654".to_string());

	let output: String = format!("{:.20?}", value);
	assert_eq!(output, "3.14159265400000009549".to_string());

	let output: String = format!("{:+.20?}", value);
	assert_eq!(output, "+3.14159265400000009549".to_string());
    }

    #[test]
    fn fmt_test2() {
	let value: T = 3.141592654;

	let output: String = format!("{:}", value);
	assert_eq!(output, "3.141592654".to_string());

	let output: String = format!("{:+}", value);
	assert_eq!(output, "+3.141592654".to_string());

	let output: String = format!("{:.20}", value);
	assert_eq!(output, "3.14159265400000009549".to_string());

	let output: String = format!("{:+.20}", value);
	assert_eq!(output, "+3.14159265400000009549".to_string());
    }

    #[test]
    fn fmt_test3() {
	let value: T = 3.141592654;

	let output: String = format!("{:e}", value);
	assert_eq!(output, "3.141592654e0".to_string());

	let output: String = format!("{:+e}", value);
	assert_eq!(output, "+3.141592654e0".to_string());

	let output: String = format!("{:.20e}", value);
	assert_eq!(output, "3.14159265400000009549e0".to_string());

	let output: String = format!("{:+.20e}", value);
	assert_eq!(output, "+3.14159265400000009549e0".to_string());
    }

    #[test]
    fn fmt_test4() {
	let value: T = 3.141592654;

	let output: String = format!("{:E}", value);
	assert_eq!(output, "3.141592654E0".to_string());

	let output: String = format!("{:+E}", value);
	assert_eq!(output, "+3.141592654E0".to_string());

	let output: String = format!("{:.20E}", value);
	assert_eq!(output, "3.14159265400000009549E0".to_string());

	let output: String = format!("{:+.20E}", value);
	assert_eq!(output, "+3.14159265400000009549E0".to_string());
    }
}
