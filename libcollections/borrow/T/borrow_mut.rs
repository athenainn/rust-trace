#![feature(core, collections)]
extern crate core;
extern crate collections;

#[cfg(test)]
mod tests {
    use collections::borrow::BorrowMut;

    use core::default::Default;

    // pub trait BorrowMut<Borrowed: ?Sized> : Borrow<Borrowed> {
    //     /// Mutably borrows from an owned value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::borrow::BorrowMut;
    //     ///
    //     /// fn check<T: BorrowMut<[i32]>>(mut v: T) {
    //     ///     assert_eq!(&mut [1, 2, 3], v.borrow_mut());
    //     /// }
    //     ///
    //     /// let v = vec![1, 2, 3];
    //     ///
    //     /// check(v);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     fn borrow_mut(&mut self) -> &mut Borrowed;
    // }

    // impl<T: ?Sized> BorrowMut<T> for T {
    //     fn borrow_mut(&mut self) -> &mut T { self }
    // }

    struct A<T> {
	value: T
    }

    impl<T> A<T> {
	fn new(value: T) -> Self {
	    A { value: value }
	}
    }

    impl<T: Default> Default for A<T> {
	fn default() -> Self {
	    Self::new(Default::default())
	}
    }

    type T = A<i32>; // T: ?Sized
    type Borrowed = T; // Borrowed: ?Sized

    #[test]
    fn borrow_mut_test1() {
	let mut x: T = T::default();

	{
	    let borrow_mut: &mut T = x.borrow_mut();
	    borrow_mut.value = 68;
	}

	assert_eq!(x.value, 68);
    }
}
