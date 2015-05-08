#![feature(core)]
#![feature(as_slice)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn as_slice_test1() {
	type T = i32;
	type E = &'static str;
	let x: Result<T, E> = Ok(2);

	assert_eq!(x.as_slice(), [2]);
	assert_eq!(x.as_slice(), &[2]);
    }
    #[test]
    fn as_slice_test2() {
	type T = [i32; 3];
	type E = &'static str;
	let x: Result<T, E> = Ok([2, 3, 4]);

	assert_eq!(x.as_slice(), [[2, 3, 4]]);
	assert_eq!(x.as_slice(), &[[2, 3, 4]]);
    }
}
