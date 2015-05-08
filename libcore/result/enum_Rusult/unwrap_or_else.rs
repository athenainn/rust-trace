#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_or_else_test() {
	type T = usize;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err("foo");

	fn count(x: E) -> T { x.len() }

	assert_eq!(x.unwrap_or_else(count), 2);
	assert_eq!(y.unwrap_or_else(count), 3);
    }
}
