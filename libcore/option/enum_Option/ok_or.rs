#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    type T = &'static str;
    type E = &'static str;
    type R = Result<T, E>;

    #[test]
    fn ok_or_test1() {
	let x: Option<T> = Some("foo");
	let a: R = x.ok_or("nothing");
	assert_eq!(a.unwrap(), "foo");
    }

    #[test]
    fn ok_or_test2() {
	let x: Option<T> = None;
	let a: R = x.ok_or("Error");
	assert_eq!(a.unwrap_err(), "Error");
    }
}
