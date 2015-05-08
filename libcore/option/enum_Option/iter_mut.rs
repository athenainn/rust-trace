#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IterMut;

    type T = u32;
    type O = Option<T>;

    #[test]
    fn iter_mut_test1() {
	let mut x: O = Some(4);

	let mut it: IterMut<T> = x.iter_mut();

	assert_eq!(it.next(), Some(&mut 4));
    }

    #[test]
    fn iter_mut_test2() {
	let mut x: O = None;

	let mut it: IterMut<T> = x.iter_mut();

	assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_mut_test3() {
	let mut x: O = Some(4);
	{
	    let mut it: IterMut<T> = x.iter_mut();

	    match it.next() {
		Some(&mut ref mut x) => *x = 42,
		None => {}
	    }
	}
	assert_eq!(x, Some(42));
    }

    #[test]
    fn iter_mut_test4() {
	let mut x: O = None;
	let mut it: IterMut<T> = x.iter_mut();
	assert_eq!(it.next(), None);
    }
}
