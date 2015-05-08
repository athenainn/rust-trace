#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn or_else_test() {
	type T = u32;
	type E = u32;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err(3);

	fn sq(x: T) -> Result<T, E> { Ok(x * x) }
	fn err(x: E) -> Result<T, E> { Err(x) }

	type F = E;
	assert_eq!(x.or_else(sq).or_else(sq), Ok::<T, F>(2));
	assert_eq!(x.or_else(err).or_else(sq), Ok::<T, F>(2));
	assert_eq!(y.or_else(sq).or_else(err), Ok::<T, F>(9));
	assert_eq!(y.or_else(err).or_else(err), Err::<T, F>(3));
    }
}
