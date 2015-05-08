#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn as_ref_test() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err("Error");

	assert_eq!(x.as_ref(), Ok(&2));
	assert_eq!(y.as_ref(), Err(&"Error"));
    }
}
