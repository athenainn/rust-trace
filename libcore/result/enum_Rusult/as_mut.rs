#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    fn mutate(r: &mut Result<i32, i32>) {
        match r.as_mut() {
            Ok(&mut ref mut v) => *v = 42,
            Err(&mut ref mut e) => *e = 0
        }
    }

    #[test]
    fn as_mut_test() {
	type T = i32;
	type E = i32;
	let mut x: Result<T, E> = Ok(2);
	let mut y: Result<T, E> = Err(13);

	assert_eq!(x.as_mut(), Ok(&mut 2));
	assert_eq!(y.as_mut(), Err(&mut 13));

	mutate(&mut x);
	assert_eq!(x.unwrap(), 42);

	mutate(&mut y);
	assert_eq!(y.unwrap_err(), 0);
    }
}
