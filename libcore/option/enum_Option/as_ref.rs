#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn as_ref_test() {
	type T = u32;
	let x: Option<T> = Some(2);
	let y: Option<T> = None;

	let a: Option<&T> = x.as_ref();
	let b: Option<&T> = y.as_ref();

	assert_eq!(a, Some(&2));
	assert_eq!(b, None);
    }
}
