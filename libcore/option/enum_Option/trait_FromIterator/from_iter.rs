#![feature(core, collections, unboxed_closures)]

extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use std::u16;
    use core::slice;
    use core::iter::Map;
    use core::iter::FromIterator;
    use collections::vec::Vec;

    type T = u16;

    struct F;

    type Args<'a> = (&'a T,);

    impl<'a> FnOnce<Args<'a>> for F {
	type Output = Option<T>;
	extern "rust-call" fn call_once(mut self, (&arg,): Args) -> Self::Output {
	    if arg == u16::MAX { None }
	    else { Some(arg + 1) }
	}
    }

    impl<'a> FnMut<Args<'a>> for F {
	extern "rust-call" fn call_mut(&mut self, (&arg,): Args) -> Self::Output {
	    if arg == u16::MAX { None }
	    else { Some(arg + 1) }
	}
    }

    #[test]
    fn from_iter_test1() {
	type V = Vec<T>;
	type I<'a> = slice::Iter<'a, T>;
	type M<'a> = Map<I<'a>, F>;

	let v: V = vec!(1, 2);
	let sit: I = v.iter();
	let f: F = F;

	let map : M = sit.map(f);
	let res: Option<V> = map.collect();

	assert!(res == Some(vec!(2, 3)));
    }

    #[test]
    fn from_iter_test2() {
	type A = T;
	type I = Vec<Option<A>>;
	type V = Vec<A>;

	let x: I = vec!(Some::<A>(2), Some::<A>(3));
	let y: Option<V> = Option::<V>::from_iter(x); // invoke x.into_iter()

	assert_eq!(y, Some::<V>(vec!(2, 3)));
    }
}
