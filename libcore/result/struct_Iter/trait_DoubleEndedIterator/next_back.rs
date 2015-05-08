#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::result::Iter;

    #[test]
    fn next_back_test() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(7);
	let y: Result<T, E> = Err("nothing!");

	let mut it1: Iter<T> = x.iter();
	let mut it2: Iter<T> = y.iter();

	let a: Option<&T> = it1.next_back();
	let b: Option<&T> = it2.next_back();

	assert_eq!(a.unwrap(), &7);
	assert_eq!(b.is_none(), true);
    }
}
