#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::option::IterMut;

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

    // pub struct IterMut<'a, A: 'a> { inner: Item<&'a mut A> }

    // impl<'a, A> ExactSizeIterator for IterMut<'a, A> {}

    type T = u32;

    #[test]
    fn len_test1() {
	let mut x: Option<T> = Some::<T>(68);
	let iter_mut: IterMut<T> = x.iter_mut();
	let len: usize = iter_mut.len();

	assert_eq!(len, 1);
    }

    #[test]
    fn len_test2() {
	let mut x: Option<T> = None::<T>;
	let iter_mut: IterMut<T> = x.iter_mut();
	let len: usize = iter_mut.len();

	assert_eq!(len, 0);
    }
}
