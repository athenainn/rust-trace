#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn map_test1() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err("Error");

	assert_eq!(x.map(|i| -i), Ok(-2));
	assert_eq!(y.map(|i| -i), Err("Error"));
    }

    #[test]
    fn map_test2() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(68);
	let y: Result<T, E> = Err("Error");

	type U = T;
	fn fn_once(x: T) -> U { -x };

	assert_eq!(x.map(fn_once), Ok(-68));
	assert_eq!(y.map(fn_once), Err("Error"));
    }

    #[test]
    fn map_test3() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(68);
	let y: Result<T, E> = Err("Error");

	type U = T;
	fn fn_once(x: T) -> U { -x };

	let a: Result<U, E> = Ok(-68);
	assert_eq!(x.map(fn_once), a);

	let b: Result<U, E> = Err("Error");
	assert_eq!(y.map(fn_once), b);
    }
}
