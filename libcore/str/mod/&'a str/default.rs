#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::default::Default;

    // impl<'a> Default for &'a str {
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn default() -> &'a str { "" }
    // }

    #[test]
    fn default_test1() {
	let x: &str = <&str>::default();

	assert_eq!(x, "");
    }

    #[test]
    fn default_test2() {
	let x: &str = Default::default();

	assert_eq!(x, "");
    }
}
