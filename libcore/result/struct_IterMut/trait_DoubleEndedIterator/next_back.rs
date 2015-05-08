#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::IterMut;

    #[test]
    fn next_back_test() {
	type T = u32;
	type E = &'static str;
	let mut x: Result<T, E> = Ok(7);
	let mut y: Result<T, E> = Err("nothing!");

	let mut it1: IterMut<T> = x.iter_mut();
	let mut it2: IterMut<T> = y.iter_mut();

	let a: Option<&mut T> = it1.next_back();
	let b: Option<&mut T> = it2.next_back();

	assert_eq!(a.unwrap(), &7);
	assert_eq!(b.is_none(), true);
    }
}
