#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn is_err_test() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(-3);
	let y: Result<T, E> = Err("Some error message");

	assert_eq!(x.is_err(), false);
	assert_eq!(y.is_err(), true);
    }
}
