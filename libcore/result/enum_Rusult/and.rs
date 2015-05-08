#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn and_test1() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);

	type U = &'static str;
	let y: Result<U, E> = Err("late error");

	assert_eq!(x.and(y), Err::<U, E>("late error"));
    }
    #[test]
    fn and_test2() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Err("early error");

	type U = &'static str;
	let y: Result<U, E> = Err("foo");

	assert_eq!(x.and(y), Err::<U, E>("early error"));
    }
    #[test]
    fn and_test3() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Err("not a 2");

	type U = &'static str;
	let y: Result<U, E> = Err("late error");

	assert_eq!(x.and(y), Err::<U, E>("not a 2"));
    }
    #[test]
    fn and_test4() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);

	type U = &'static str;
	let y: Result<U, E> = Ok("different result type");

	assert_eq!(x.and(y), Ok::<U, E>("different result type"));
    }
}
