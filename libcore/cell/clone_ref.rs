#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use core::cell::Ref;
    use core::cell::clone_ref;

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

    // pub struct Ref<'b, T: ?Sized + 'b> {
    //     // FIXME #12808: strange name to try to avoid interfering with
    //     // field accesses of the contained type via Deref
    //     _value: &'b T,
    //     _borrow: BorrowRef<'b>,
    // }

    // impl<'b, T: ?Sized> Deref for Ref<'b, T> {
    //     type Target = T;
    //
    //     #[inline]
    //     fn deref<'a>(&'a self) -> &'a T {
    //         self._value
    //     }
    // }

    // pub fn clone_ref<'b, T:Clone>(orig: &Ref<'b, T>) -> Ref<'b, T> {
    //     Ref {
    //         _value: orig._value,
    //         _borrow: orig._borrow.clone(),
    //     }
    // }

    type T = i32;

    #[test]
    fn clone_ref_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let value_ref: Ref<T> = refcell.borrow();
	let clone_ref: Ref<T> = clone_ref(&value_ref);

	assert_eq!(*value_ref, *clone_ref);
	assert_eq!(*value_ref, value);
    }
}
