#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::Iter;

    type T = u32;
    type O = Option<T>;
    type A = T;

    #[test]
    fn next_test1() {

	let x: O = Some(7);
	let mut it: Iter<T> = x.iter();
	let a: Option<&A> = it.next();

	assert_eq!(a.unwrap(), &7);
    }

    #[test]
    fn next_test2() {
	let x: O = None;

	type A = T;
	let mut it: Iter<T> = x.iter();
	let a: Option<&A> = it.next();

	assert_eq!(a.is_none(), true);
    }
}
