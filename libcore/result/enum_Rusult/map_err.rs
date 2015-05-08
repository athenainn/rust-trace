#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn map_err_test1() {
	type T = u32;
	type E = u32;
	let x: Result<T, E> = Ok(2);
	let y: Result<T, E> = Err(13);

	assert_eq!(x.map_err(|i| format!("error code: {}", i) ), Ok(2));
        assert_eq!(y.map_err(|i| format!("error code: {}", i) ), Err("error code: 13".to_string()));
    }

    #[test]
    fn map_err_test2() {
	type T = i32;
	type E = u32;
	let x: Result<T, E> = Ok(68);
	let y: Result<T, E> = Err(500);

	type F = String;
	fn fn_once(x: E) -> F { format!("error code: {}", x) };

	let a: Result<T, F> = Ok(68);

	assert_eq!(x.map_err(fn_once), a);
	assert_eq!(y.map_err(fn_once), Err("error code: 500".to_string()));
    }
}
