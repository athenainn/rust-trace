#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn err_test() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err("Nothing here");

	assert_eq!(x.err(), None);
	assert_eq!(y.err(), Some("Nothing here"));
    }
}
