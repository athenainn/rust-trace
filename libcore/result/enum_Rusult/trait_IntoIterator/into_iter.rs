#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use std::u32;

    #[test]
    fn into_iter_test() {
	type T = u32;
	type E = &'static str;
	let x: Result<T, E> = Ok(5);
	let y: Result<T, E> = Err("nothing!");

	assert_eq!(x.into_iter().next(), Some(5));
	assert_eq!(y.into_iter().next(), None);

	assert_eq!(x.into_iter().count(), 1);
	assert_eq!(y.into_iter().count(), 0);

	let v: Vec<T> = x.into_iter().collect();
	assert_eq!(v, [5]);

	let w: Vec<T> = y.into_iter().collect();
	assert_eq!(w, []);
    }
}
