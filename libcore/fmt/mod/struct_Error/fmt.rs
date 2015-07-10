#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::fmt::Error;

    // #[derive(Copy, Clone, Debug)]
    // pub struct Error;

    // impl Display for Error {
    //     fn fmt(&self, f: &mut Formatter) -> Result {
    //         Display::fmt("an error occurred when formatting an argument", f)
    //     }
    // }

    #[test]
    fn fmt_test1() {
	let error: Error = Error;
	let output: String = format!("{}", error);

	assert_eq!(output, "an error occurred when formatting an argument".to_string());
    }
}
