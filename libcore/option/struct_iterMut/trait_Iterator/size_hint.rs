#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IterMut;

    type T = u32;
    type O = Option<T>;
    type A = T;

    #[test]
    fn size_hint_test1() {
	let mut x: O = Some(68);
	let it: IterMut<T> = x.iter_mut();
	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 1);
	assert_eq!(a.1, Some::<usize>(1));
    }

    #[test]
    fn size_hint_test2() {
	let mut x: O = None;
	let it: IterMut<T> = x.iter_mut();
	let a: (usize, Option<usize>) = it.size_hint();

	assert_eq!(a.0, 0);
	assert_eq!(a.1, Some::<usize>(0));
    }
}
