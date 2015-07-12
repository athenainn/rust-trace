#![feature(alloc)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    // pub struct Box<T>(Unique<T>);

    // impl<T: Clone> Clone for Box<T> {
    //     /// Returns a new box with a `clone()` of this box's contents.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// let x = Box::new(5);
    //     /// let y = x.clone();
    //     /// ```
    //     #[inline]
    //     fn clone(&self) -> Box<T> { box {(**self).clone()} }
    //
    //     /// Copies `source`'s contents into `self` without creating a new allocation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(box_raw)]
    //     /// let x = Box::new(5);
    //     /// let mut y = Box::new(10);
    //     ///
    //     /// y.clone_from(&x);
    //     ///
    //     /// assert_eq!(*y, 5);
    //     /// ```
    //     #[inline]
    //     fn clone_from(&mut self, source: &Box<T>) {
    //         (**self).clone_from(&(**source));
    //     }
    // }

    type T = i32; // T: Clone

    #[test]
    fn clone_from_test1() {
	let x: T = 68;
	let source: Box<T> = Box::<T>::new(x);

	let mut b: Box<T> = Box::<T>::default();
	b.clone_from(&source);

	assert_eq!(*b, 68);
    }
}
