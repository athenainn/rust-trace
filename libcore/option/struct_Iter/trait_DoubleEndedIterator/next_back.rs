#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::Iter;

    type T = u32;
    type O = Option<T>;
    type A = T;

    #[test]
    fn next_back_test1() {
	let x: O = Some(68);

	let mut it1: Iter<T> = x.iter();
	let a: Option<&A> = it1.next_back();

	assert_eq!(a.unwrap(), &68);
    }

    #[test]
    fn next_back_test2() {
	let x: O = None;

	let mut it1: Iter<T> = x.iter();
	let a: Option<&T> = it1.next_back();

	assert_eq!(a, None);
    }
}
