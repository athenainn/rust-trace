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

    // impl<A> ExactSizeIterator for IntoIter<A> {}

    type T = u32;

    #[test]
    fn len_test1() {
	let x: Option<T> = Some::<T>(7);
	let into_iter: IntoIter<T> = x.into_iter();
	let len: usize = into_iter.len();

	assert_eq!(len, 1);
    }

    #[test]
    fn len_test2() {
	let x: Option<T> = None::<T>;
	let into_iter: IntoIter<T> = x.into_iter();
	let len: usize = into_iter.len();

	assert_eq!(len, 0);
    }
}
