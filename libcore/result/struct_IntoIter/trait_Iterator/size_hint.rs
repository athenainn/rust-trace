#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::IntoIter;

    #[test]
    fn size_hint_test1() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(7);

	let it: IntoIter<T> = x.into_iter();

	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 1);
	assert_eq!(a.1, Some::<usize>(1));
    }

    #[test]
    fn size_hint_test2() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Err("nothing!");

	let it: IntoIter<T> = x.into_iter();

	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 0);
	assert_eq!(a.1, Some::<usize>(0));
    }
}
