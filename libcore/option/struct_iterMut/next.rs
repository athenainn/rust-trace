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

    // impl<'a, A> Iterator for IterMut<'a, A> {
    //     type Item = &'a mut A;
    //
    //     #[inline]
    //     fn next(&mut self) -> Option<&'a mut A> { self.inner.next() }
    //     #[inline]
    //     fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
    // }

    type T = u32;

    #[test]
    fn next_test1() {
	let mut x: Option<T> = Some::<T>(68);

	{
	    let mut iter: IterMut<T> = x.iter_mut();
	    let result: Option<&mut T> = iter.next();

	    match result {
		Some(&mut ref mut x) => *x = 500,
		None => assert!(false)
	    }
	}

	assert_eq!(x, Some::<T>(500));
    }

    #[test]
    fn next_test2() {
	let mut x: Option<T> = None::<T>;
	let mut iter: IterMut<T> = x.iter_mut();
	let result: Option<&mut T> = iter.next();

	match result {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}
    }
}
