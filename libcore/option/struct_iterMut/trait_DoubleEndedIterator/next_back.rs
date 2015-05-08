#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IterMut;

    type T = u32;
    type O = Option<T>;
    type A = T;

    #[test]
    fn next_back_test1() {
	let mut x: O = Some(68);
	let mut it: IterMut<T> = x.iter_mut();
	let a: Option<&mut A> = it.next_back();

	assert_eq!(a.unwrap(), &68);
    }

    #[test]
    fn next_back_test2() {
	let mut x: O = None;
	let mut it: IterMut<T> = x.iter_mut();
	let a: Option<&mut A> = it.next_back();

	assert_eq!(a.is_none(), true);
    }
}
