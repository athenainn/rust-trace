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

    // impl<'a, A> ExactSizeIterator for Iter<'a, A> {}

    type T = u32;

    #[test]
    fn len_test1() {
	let x: Option<T> = Some::<T>(7);
	let iter: Iter<T> = x.iter();
	let len: usize = iter.len();

	assert_eq!(len, 1);
    }

    #[test]
    fn len_test2() {
	let x: Option<T> = None::<T>;
	let iter: Iter<T> = x.iter();
	let len: usize = iter.len();

	assert_eq!(len, 0);
    }
}
