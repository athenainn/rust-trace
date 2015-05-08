#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn ok_test() {
	type T = i32;
	type E = &'static str;
	type R = Result<T, E>;
	type O = Option<T>;

	let x: R = Ok(2);
	let y: R = Err("Nothing here");

	assert_eq!(x.ok(), Some(2));
	assert_eq!(y.ok(), None);

	let a: O = Some(2);
	assert_eq!(x.ok(), a);

	let b: O = None;
	assert_eq!(y.ok(), b);
    }
}
