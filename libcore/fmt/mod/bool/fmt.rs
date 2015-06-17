#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    // impl Debug for bool {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         Display::fmt(self, f)
    //     }
    // }

    // impl Display for bool {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         Display::fmt(if *self { "true" } else { "false" }, f)
    //     }
    // }

    #[test]
    fn fmt_test1() {
	let value: bool = false;
	let output: String = format!("{}", value);

	assert_eq!(output, "false".to_string());
    }

    #[test]
    fn fmt_test2() {
	let value: bool = true;
	let output: String = format!("{}", value);

	assert_eq!(output, "true".to_string());
    }

    #[test]
    fn fmt_test3() {
	let value: bool = false;
	let output: String = format!("{:?}", value);

	assert_eq!(output, "false".to_string());
    }

    #[test]
    fn fmt_test4() {
	let value: bool = true;
	let output: String = format!("{:?}", value);

	assert_eq!(output, "true".to_string());
    }
}
