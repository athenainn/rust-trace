#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::IterMut;

    #[test]
    fn size_hint_test() {
	type T = u32;
	type E = &'static str;
	let mut x: Result<T, E> = Ok(7);
	let mut y: Result<T, E> = Err("nothing!");

	let it1: IterMut<T> = x.iter_mut();
	let it2: IterMut<T> = y.iter_mut();

	let a: (usize, Option<usize>) = it1.size_hint();
	let b: (usize, Option<usize>) = it2.size_hint();

	assert_eq!(a.0, 1);
	assert_eq!(a.1, Some::<usize>(1));
	assert_eq!(b.0, 0);
	assert_eq!(b.1, Some::<usize>(0));
    }
}
