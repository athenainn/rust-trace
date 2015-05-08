#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn expect_test1() {
	type T = &'static str;
	let x: Option<T> = Some("value");

	assert_eq!(x.expect("the world is ending"), "value");
    }

    #[test]
    #[should_panic]
    fn expect_test2() {
	type T = &'static str;
	let x: Option<T> = None;

	x.expect("the world is ending"); // panic
    }
}
