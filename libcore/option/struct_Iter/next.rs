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
    fn next_test1() {
	let x: Option<T> = Some::<T>(7);
	let mut iter: Iter<T> = x.iter();
	let result: Option<&T> = iter.next();

	match result {
	    Some(v) => assert_eq!(*v, 7),
	    None => assert!(false)
	}

	assert_eq!(iter.next(), None::<&T>);
    }

    #[test]
    fn next_test2() {
	let x: Option<T> = None::<T>;
	let mut iter: Iter<T> = x.iter();
	let result: Option<&T> = iter.next();

	match result {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}
    }
}
