#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::IntoIter;

    #[test]
    fn next_test1() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(7);

	let mut it: IntoIter<T> = x.into_iter();

	let a: Option<T> = it.next();

	assert_eq!(a, Some::<T>(7));
    }

    #[test]
    fn next_test2() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Err("nothing!");

	let mut it: IntoIter<T> = x.into_iter();

	let a: Option<T> = it.next();

	assert_eq!(a, None);
    }
}
