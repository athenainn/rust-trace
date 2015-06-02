#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::default::Default;

    // impl<'a, T> Default for &'a [T] {
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn default() -> &'a [T] { &[] }
    // }

    type T = i32;

    #[test]
    fn default_test1() {
	let default: &[T] = <&[T]>::default();

	assert_eq!(default, &[]);
    }

    #[test]
    fn default_test2() {
	let default: &[T] = Default::default();

	assert_eq!(default, &[]);
    }
}
