#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::iter::MinMaxResult::{self, NoElements, OneElement, MinMax};

    type T = i32; // T: Clone

    #[test]
    fn into_option_test1() {
	let x: MinMaxResult<T> = NoElements::<T>;
	let y: Option<(T, T)> = x.into_option();

	assert_eq!(y, None::<(T, T)>);
    }

    #[test]
    fn into_option_test2() {
	let x: MinMaxResult<T> = OneElement::<T>(68);
	let y: Option<(T, T)> = x.into_option();

	assert_eq!(y, Some::<(T, T)>((68, 68)));
    }

    #[test]
    fn into_option_test3() {
	let x: MinMaxResult<T> = MinMax::<T>(68, 500);
	let y: Option<(T, T)> = x.into_option();

	assert_eq!(y, Some::<(T, T)>((68, 500)));
    }
}
