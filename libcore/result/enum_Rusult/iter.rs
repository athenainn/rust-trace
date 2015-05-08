#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::Iter;

    #[test]
    fn iter_test() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(7);
	let y: Result<T, E> = Err("nothing!");

	assert_eq!(x.iter().next(), Some(&7));
	assert_eq!(y.iter().next(), None);

	let a: Iter<T> = x.iter();
	let b: Iter<T> = y.iter();

	assert_eq!(a.count(), 1);
	assert_eq!(b.count(), 0);
    }
}
