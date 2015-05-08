#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::Iter;

    type T = u32;
    type O = Option<T>;

    #[test]
    fn size_hint_test1() {
	let x: O = Some(68);
	let it: Iter<T> = x.iter();
	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 1);
	assert_eq!(a.1, Some::<usize>(1));
    }

    #[test]
    fn size_hint_test2() {
	let x: O = None;
	let it: Iter<T> = x.iter();
	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 0);
	assert_eq!(a.1, Some::<usize>(0));
    }
}
