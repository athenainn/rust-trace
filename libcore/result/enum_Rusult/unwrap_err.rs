#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn unwrap_err_test1() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err("emergency failure");

	x.unwrap_err(); // panics with `2`
	assert_eq!(y.unwrap_err(), "emergency failure");
    }

    #[test]
    #[should_panic]
    fn unwrap_test2() {
	#[derive(Debug)]
	struct A { value: u32 }

	type T = A;
	type E = &'static str;
	let x: Result<T, E> = Ok(A { value: 2});
	let y: Result<T, E> = Err("emergency failure");

	x.unwrap_err(); // panics with `A { value: 2 }`
	assert_eq!(y.unwrap_err(), "emergency failure");
    }
}
