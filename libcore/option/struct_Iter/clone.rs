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

    // impl<'a, A> Clone for Iter<'a, A> {
    //     fn clone(&self) -> Iter<'a, A> {
    //         Iter { inner: self.inner.clone() }
    //     }
    // }

    type T = u32;

    #[test]
    fn clone_test1() {
	let x: Option<T> = Some::<T>(68);
	let mut iter: Iter<T> = x.iter();
	let mut clone: Iter<T> = iter.clone();

	assert_eq!(iter.next(), Some::<&T>(&68));
	assert_eq!(iter.next(), None::<&T>);

	assert_eq!(clone.next(), Some::<&T>(&68));
	assert_eq!(clone.next(), None::<&T>);
    }

    #[test]
    fn clone_test2() {
	let x: Option<T> = None::<T>;
	let mut iter: Iter<T> = x.iter();
	let mut clone: Iter<T> = iter.clone();

	assert_eq!(iter.next(), None::<&T>);

	assert_eq!(clone.next(), None::<&T>);
    }
}
