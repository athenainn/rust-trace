#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IntoIter;

    type T = u32;
    type A = T;

    #[test]
    fn next_test1() {
	let x: Option<T> = Some::<T>(7);

	let mut it: IntoIter<A> = x.into_iter();

	let a: Option<A> = it.next();

	assert_eq!(a, Some::<A>(7));
    }
    #[test]
    fn next_test2() {
	let x: Option<T> = None::<T>;

	let mut it: IntoIter<A> = x.into_iter();

	let a: Option<A> = it.next();

	assert_eq!(a, None::<A>);
    }
}
