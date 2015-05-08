#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IntoIter;

    type T = u32;

    #[test]
    fn size_hint_test1() {
	let x: Option<T> = Some::<T>(7);

	let it: IntoIter<T> = x.into_iter();

	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 1);
	assert_eq!(a.1, Some::<usize>(1));
    }
    #[test]
    fn size_hint_test2() {
	let x: Option<T> = None::<T>;

	let it: IntoIter<T> = x.into_iter();

	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 0);
	assert_eq!(a.1, Some::<usize>(0));
    }
}
