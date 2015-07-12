#![feature(core, core_slice_ext, alloc)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use core::slice::SliceExt;
    use core::slice::Iter;

    // pub struct Box<T>(Unique<T>);

    // impl<T> Box<T> {
    //     /// Allocates memory on the heap and then moves `x` into it.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Box::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline(always)]
    //     pub fn new(x: T) -> Box<T> {
    //         box x
    //     }
    // }

    // impl<I: Iterator + ?Sized> Iterator for Box<I> {
    //     type Item = I::Item;
    //     fn next(&mut self) -> Option<I::Item> { (**self).next() }
    //     fn size_hint(&self) -> (usize, Option<usize>) { (**self).size_hint() }
    // }

    type T = i32;
    type I<'a> = Iter<'a, T>; // Iterator + ?Sized

    #[test]
    fn next_test1() {
	let slice: &[T] = &[0, 1, 2, 3, 4, 5];
	let iter: I = slice.iter();
	let mut b: Box<I> = Box::<I>::new(iter);

	let (lower, upper): (usize, Option<usize>) = b.size_hint();
	assert_eq!(lower, 6);
	assert_eq!(upper, Some::<usize>(6));

	for n in 0..lower {
	    let result: Option<&T> = b.next();

	    match result {
		Some(&v) => assert_eq!(v, n as T),
		None => assert!(false)
	    }
	}

	assert_eq!(b.next(), None::<&T>);
    }
}
