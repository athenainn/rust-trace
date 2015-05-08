#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::Iter;

    #[test]
    fn size_hint_test() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(7);
	let y: Result<T, E> = Err("nothing!");

	let it1: Iter<T> = x.iter();
	let it2: Iter<T> = y.iter();

	let a: (usize, Option<usize>) = it1.size_hint();
	let b: (usize, Option<usize>) = it2.size_hint();

	assert_eq!(a.0, 1);
	assert_eq!(a.1, Some::<usize>(1));
	assert_eq!(b.0, 0);
	assert_eq!(b.1, Some::<usize>(0));
    }
}
