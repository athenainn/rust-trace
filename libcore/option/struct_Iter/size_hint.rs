#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::Iter;

    // #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub enum Option<T> {
    //     /// No value
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     None,
    //     /// Some value `T`
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Some(T)
    // }

    // pub struct Iter<'a, A: 'a> { inner: Item<&'a A> }

    // impl<'a, A> Iterator for Iter<'a, A> {
    //     type Item = &'a A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<&'a A> { self.inner.next() }
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
    // }

    type T = u32;

    #[test]
    fn size_hint_test1() {
	let x: Option<T> = Some::<T>(7);
	let iter: Iter<T> = x.iter();
	let (lower, upper): (usize, Option<usize>) = iter.size_hint();

	assert_eq!(lower, 1);
	assert_eq!(upper, Some::<usize>(1));
    }

    #[test]
    fn size_hint_test2() {
	let x: Option<T> = None::<T>;
	let iter: Iter<T> = x.iter();
	let (lower, upper): (usize, Option<usize>) = iter.size_hint();

	assert_eq!(lower, 0);
	assert_eq!(upper, Some::<usize>(0));
    }
}
