#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    #[test]
    fn is_some_test() {
	type T = u32;
	let x: Option<T> = Some(2);
	let y: Option<T> = None;

	assert_eq!(x.is_some(), true);
	assert_eq!(y.is_some(), false);
    }
}
