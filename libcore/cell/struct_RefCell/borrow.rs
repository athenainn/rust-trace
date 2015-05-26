#![feature(core, std_misc)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use core::cell::BorrowState::{self, Reading, Writing, Unused};
    use core::cell::Ref;
    use core::cell::RefMut;

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

    // pub enum BorrowState {
    //     /// The cell is currently being read, there is at least one active `borrow`.
    //     Reading,
    //     /// The cell is currently being written to, there is an active `borrow_mut`.
    //     Writing,
    //     /// There are no outstanding borrows on this cell.
    //     Unused,
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

    // pub struct RefMut<'b, T: ?Sized + 'b> {
    //     // FIXME #12808: strange name to try to avoid interfering with
    //     // field accesses of the contained type via Deref
    //     _value: &'b mut T,
    //     _borrow: BorrowRefMut<'b>,
    // }

    // impl<'b, T: ?Sized> Deref for RefMut<'b, T> {
    //     type Target = T;
    //
    //     #[inline]
    //     fn deref<'a>(&'a self) -> &'a T {
    //         self._value
    //     }
    // }

    type T = i32;

    #[test]
    fn borrow_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);

	{
	    let value_ref: Ref<T> = refcell.borrow();
	    assert_eq!(*value_ref, value);

	    let borrow_state: BorrowState = refcell.borrow_state();
	    assert_eq!(borrow_state, Reading);
	}

	let borrow_state: BorrowState = refcell.borrow_state();
	assert_eq!(borrow_state, Unused);
    }

    #[test]
    fn borrow_test2() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);

	{
	    let value_ref1: Ref<T> = refcell.borrow();
	    assert_eq!(*value_ref1, value);

	    let borrow_state1: BorrowState = refcell.borrow_state();
	    assert_eq!(borrow_state1, Reading);

	    let value_ref2: Ref<T> = refcell.borrow();
	    assert_eq!(*value_ref2, value);

	    let borrow_state2: BorrowState = refcell.borrow_state();
	    assert_eq!(borrow_state2, Reading);
	}

	let borrow_state: BorrowState = refcell.borrow_state();
	assert_eq!(borrow_state, Unused);
    }

    #[test]
    #[should_panic]
    fn borrow_test3() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);

	let value_refmut: RefMut<T> = refcell.borrow_mut();
	assert_eq!(*value_refmut, value);

	let borrow_state1: BorrowState = refcell.borrow_state();
	assert_eq!(borrow_state1, Writing);

	let _: Ref<T> = refcell.borrow(); // panicked at 'RefCell<T> already mutably borrowed'
    }
}
