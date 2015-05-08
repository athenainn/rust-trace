#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::IterMut;

    #[test]
    fn iter_mut_test() {
	type T = u32;
	type E = &'static str;
	let mut x: Result<T, E> = Ok(7);
	let mut y: Result<T, E> = Err("nothing!");

	assert_eq!(x.iter_mut().next(), Some(&mut 7));
	assert_eq!(y.iter_mut().next(), None);

	match x.iter_mut().next() {
	    Some(&mut ref mut x) => *x = 40,
	    None => {}
	}
	assert_eq!(x, Ok(40));
	assert_eq!(y.iter_mut().next(), None);

	let a: IterMut<T> = x.iter_mut();
	let b: IterMut<T> = y.iter_mut();

	assert_eq!(a.count(), 1);
	assert_eq!(b.count(), 0);
    }
}
