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

    // impl<A> Iterator for IntoIter<A> {
    //     type Item = A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<A> { self.inner.next() }
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
    // }

    type T = u32;

    #[test]
    fn size_hint_test1() {
	let x: Option<T> = Some::<T>(7);
	let into_iter: IntoIter<T> = x.into_iter();
	let (lower, upper): (usize, Option<usize>) = into_iter.size_hint();

	assert_eq!(lower, 1);
	assert_eq!(upper, Some::<usize>(1));
    }

    #[test]
    fn size_hint_test2() {
	let x: Option<T> = None::<T>;
	let mut n: usize = 0;

	for v in x  {
	    n += 1;
	    assert!(false);	
	}

	assert_eq!(n, 0);
    }
}
