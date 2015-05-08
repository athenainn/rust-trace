#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_or_test1() {
	type T = &'static str;
	let x: Option<T> = Some("car");

	assert_eq!(x.unwrap_or("bike"), "car");
    }

    #[test]
    fn unwrap_or_test2() {
	type T = &'static str;
	let x: Option<T> = None;

	assert_eq!(x.unwrap_or("bike"), "bike");
    }
}
