#![feature(core, alloc)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use core::ops::DerefMut;

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

    // impl<T: ?Sized> Deref for Box<T> {
    //     type Target = T;
    //
    //     fn deref(&self) -> &T { &**self }
    // }

    type T = i32; // T: ?Sized

    #[test]
    fn deref_mut_test1() {
	let x: T = 68;
	let mut b: Box<T> = Box::<T>::new(x);

	{
	    let deref_mut: &mut T = b.deref_mut();

	    *deref_mut = 500;
	}

	assert_eq!(*b, 500);
    }
}
