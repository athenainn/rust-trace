#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn and_then_test() {
	type T = u32;
	type E = u32;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err(3);

	fn sq(x: T) -> Result<T, E> { Ok(x * x) }
	fn err(x: E) -> Result<T, E> { Err(x) }

	type U = T;
	assert_eq!(x.and_then(sq).and_then(sq), Ok::<U, E>(16));
	assert_eq!(x.and_then(sq).and_then(err), Err::<U, E>(4));
	assert_eq!(x.and_then(err).and_then(sq), Err::<U, E>(2));
	assert_eq!(y.and_then(sq).and_then(sq), Err::<U, E>(3));
    }
}
