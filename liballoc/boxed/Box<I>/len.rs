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

    // impl<I: ExactSizeIterator + ?Sized> ExactSizeIterator for Box<I> {}

    type T = i32;
    type I<'a> = Iter<'a, T>; // Iterator + ?Sized

    #[test]
    fn len_test1() {
	let slice: &[T] = &[0, 1, 2, 3, 4, 5];
	let iter: I = slice.iter();
	let b: Box<I> = Box::<I>::new(iter);

	let len: usize = b.len();
	assert_eq!(len, 6);
    }
}
