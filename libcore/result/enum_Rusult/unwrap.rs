#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn unwrap_test1() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err("emergency failure");

	assert_eq!(x.unwrap(), 2);
	y.unwrap(); // panics with `emergency failure`
    }

    #[test]
    #[should_panic]
    fn unwrap_test2() {
	#[derive(Debug)]
	struct A { value: u32 }

	type T = u32;
	type E = A;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err(A { value: 68});

	assert_eq!(x.unwrap(), 2);
	y.unwrap(); // panics with `emergency failure`
    }
}
