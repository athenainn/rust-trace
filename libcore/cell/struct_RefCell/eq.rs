#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    // pub struct RefCell<T: ?Sized> {
    //     borrow: Cell<BorrowFlag>,
    //     value: UnsafeCell<T>,
    // }

    // impl<T> RefCell<T> {
    //     /// Creates a new `RefCell` containing `value`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::RefCell;
    //     ///
    //     /// let c = RefCell::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn new(value: T) -> RefCell<T> {
    //         RefCell {
    //             value: UnsafeCell::new(value),
    //             borrow: Cell::new(UNUSED),
    //         }
    //     }
    //
    //     /// Consumes the `RefCell`, returning the wrapped value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::RefCell;
    //     ///
    //     /// let c = RefCell::new(5);
    //     ///
    //     /// let five = c.into_inner();
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn into_inner(self) -> T {
    //         // Since this function takes `self` (the `RefCell`) by value, the
    //         // compiler statically verifies that it is not currently borrowed.
    //         // Therefore the following assertion is just a `debug_assert!`.
    //         debug_assert!(self.borrow.get() == UNUSED);
    //         unsafe { self.value.into_inner() }
    //     }
    // }

    // impl<T: ?Sized + PartialEq> PartialEq for RefCell<T> {
    //     #[inline]
    //     fn eq(&self, other: &RefCell<T>) -> bool {
    //         *self.borrow() == *other.borrow()
    //     }
    // }

    type T = i32;

    #[test]
    fn eq_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let clone: RefCell<T> = refcell.clone();

	assert_eq!(refcell.eq(&clone), true);
    }

    #[test]
    fn eq_test2() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let clone: RefCell<T> = refcell.clone();

	assert_eq!(refcell == clone, true);
    }

    #[test]
    fn eq_test3() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let default: RefCell<T> = RefCell::<T>::default();

	assert_eq!(refcell.eq(&default), false);
    }

    #[test]
    fn eq_test4() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let default: RefCell<T> = RefCell::<T>::default();

	assert_eq!(refcell == default, false);
    }
}
