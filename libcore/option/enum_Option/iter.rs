#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::Iter;

    type T = u32;
    type O = Option<T>;

    #[test]
    fn iter_test1() {
	let x: O = Some(4);

	let mut it: Iter<T> = x.iter();

	assert_eq!(it.next(), Some(&4));
    }

    #[test]
    fn iter_test2() {
	let x: O = None;

	let mut it: Iter<T> = x.iter();

	assert_eq!(it.next(), None);
    }
}
