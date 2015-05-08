#![feature(core)]
#![feature(as_slice)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn as_slice_test1() {
	type T = i32;
	let x: Option<T> = Some(2);
	let y: &[T] = x.as_slice();

	assert_eq!(y, [2]);
	assert_eq!(y, &[2]);
    }
    #[test]
    fn as_slice_test2() {
	type T = [i32; 3];
	let x: Option<T> = Some([2, 3, 4]);
	let y: &[T] = x.as_slice();

	assert_eq!(y, [[2, 3, 4]]);
	assert_eq!(y, &[[2, 3, 4]]);
    }
}
