#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::Iter;

    type T = u32;
    type O = Option<T>;
    type A = T;

    #[test]
    fn clone_test1() {
	let x: O = Some(68);
	let mut it1: Iter<T> = x.iter();
	let mut it2: Iter<A> = it1.clone();

	assert_eq!(it1.clone().next(), Some(&68));
	assert_eq!(it1.next(), it2.next());
    }

    #[test]
    fn clone_test2() {
	let x: O = None;
	let mut it1: Iter<T> = x.iter();
	let mut it2: Iter<A> = it1.clone();

	assert_eq!(it1.clone().next(), None);
	assert_eq!(it1.next(), it2.next());
    }
}
