#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn or_test1() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);

	type F = E;
	let y: Result<T, F> = Err("late error");

	assert_eq!(x.or(y), Ok::<T, F>(2));
    }
    #[test]
    fn or_test2() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Err("early error");

	type F = E;
	let y: Result<T, F> = Ok(2);

	assert_eq!(x.or(y), Ok::<T, F>(2));
    }
    #[test]
    fn or_test3() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Err("not a 2");

	type F = E;
	let y: Result<T, F> = Err("late error");

	assert_eq!(x.or(y), Err::<T, F>("late error"));
    }
    #[test]
    fn or_test4() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);

	type F = E;
	let y: Result<T, F> = Ok(100);

	assert_eq!(x.or(y), Ok::<T, F>(2));
    }
}
