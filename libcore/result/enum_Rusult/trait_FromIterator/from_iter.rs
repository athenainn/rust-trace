#![feature(core)]
#![feature(collections)]

extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use std::u32;
    use core::slice;
    use core::iter::Map;
    use core::iter::FromIterator;
    use collections::vec::Vec;

    #[test]
    fn from_iter_test1() {
	type T = u32;
	type E = &'static str;
	type R = Result<T, E>;
	type F = fn(&T) -> R;
	type V = Vec<T>;
	type I<'a> = slice::Iter<'a, T>;
	type M<'a> = Map<I<'a>, F>;

	let v: V = vec!(1, 2);
	let sit: I = v.iter();

	fn fn_once(&x: &T) -> R {
	    if x == u32::MAX { Err("Overflow!") }
	    else { Ok(x + 1) }
	}

	let map : M = sit.map(fn_once);
	let res: Result<V, E> = map.collect();

	assert!(res == Ok(vec!(2, 3)));
    }

    #[test]
    fn from_iter_test2() {
	type T = u32;
	type A = T;
	type E = &'static str;
	type R = Result<A, E>;
	type I = Vec<R>;
	type V = Vec<A>;

	let x: I = vec!(Ok::<A, E>(2), Ok::<A, E>(3));
	let y: Result<V, E> = Result::<V, E>::from_iter(x); // invoke x.into_iter()

	assert_eq!(y, Ok::<V, E>(vec!(2, 3)));
    }
}
