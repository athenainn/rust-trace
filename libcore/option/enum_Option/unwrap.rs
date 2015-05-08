#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_test1() {
	type T = &'static str;
	let x: Option<T> = Some("air");

	assert_eq!(x.unwrap(), "air");
    }

    #[test]
    #[should_panic]
    fn unwrap_test2() {
	type T = &'static str;
	let x: Option<T> = None;

	x.unwrap(); // panic
    }
}
