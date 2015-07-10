#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IntoIter;

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

    // pub struct IntoIter<A> { inner: Item<A> }

    // impl<A> DoubleEndedIterator for IntoIter<A> {
    //     #[inline]
    //     fn next_back(&mut self) -> Option<A> { self.inner.next_back() }
    // }

    type T = u32;

    #[test]
    fn next_back_test1() {
	let x: Option<T> = Some::<T>(7);
	let mut into_iter: IntoIter<T> = x.into_iter();
	let result: Option<T> = into_iter.next_back();

	assert_eq!(result, Some::<T>(7));

	assert_eq!(into_iter.next_back(), None::<T>);
    }

    #[test]
    fn next_back_test2() {
	let x: Option<T> = None::<T>;
	let mut into_iter: IntoIter<T> = x.into_iter();
	let result: Option<T> = into_iter.next_back();

	assert_eq!(result, None::<T>);
    }
}
