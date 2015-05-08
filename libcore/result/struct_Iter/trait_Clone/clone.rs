#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::Iter;

    #[test]
    fn clone_test() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(7);
	let y: Result<T, E> = Err("nothing!");

	let mut it1: Iter<T> = x.iter();
	let mut it2: Iter<T> = y.iter();

	let mut it3: Iter<T> = it1.clone();
	let mut it4: Iter<T> = it2.clone();

	assert_eq!(it1.clone().next().unwrap(), &7);
	assert_eq!(it1.next(), it3.next());
	assert_eq!(it2.next().is_none(), true);
	assert_eq!(it2.next(), it4.next());
    }
}
