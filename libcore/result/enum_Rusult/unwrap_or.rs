#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_or_test() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(9);
	let y: Result<T, E> = Err("error");
	let optb = 2;

	assert_eq!(x.unwrap_or(optb), 9);
	assert_eq!(y.unwrap_or(optb), optb);
    }
}
