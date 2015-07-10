#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use core::cell::UnsafeCell;
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

    // pub struct UnsafeCell<T: ?Sized> {
    //     /// Wrapped value
    //     ///
    //     /// This field should not be accessed directly, it is made public for static
    //     /// initializers.
    //     #[unstable(feature = "core")]
    //     pub value: T,
    // }

    // impl<T> UnsafeCell<T> {
    //     /// Constructs a new instance of `UnsafeCell` which will wrap the specified
    //     /// value.
    //     ///
    //     /// All access to the inner value through methods is `unsafe`, and it is highly discouraged to
    //     /// access the fields directly.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::UnsafeCell;
    //     ///
    //     /// let uc = UnsafeCell::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn new(value: T) -> UnsafeCell<T> {
    //         UnsafeCell { value: value }
    //     }
    //
    //     /// Unwraps the value.
    //     ///
    //     /// # Unsafety
    //     ///
    //     /// This function is unsafe because there is no guarantee that this or other threads are
    //     /// currently inspecting the inner value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::UnsafeCell;
    //     ///
    //     /// let uc = UnsafeCell::new(5);
    //     ///
    //     /// let five = unsafe { uc.into_inner() };
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub unsafe fn into_inner(self) -> T { self.value }
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
    fn as_unsafe_call_test1() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);

	let unsafecell: &UnsafeCell<T> = unsafe { refcell.as_unsafe_cell() };
	assert_eq!(unsafecell.value, value);

	unsafe { *unsafecell.get() = 500; };
	assert_eq!(unsafecell.value, 500);
    }

    #[test]
    fn as_unsafe_call_test2() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let value_ref: Ref<T> = refcell.borrow();

	let unsafecell: &UnsafeCell<T> = unsafe { refcell.as_unsafe_cell() };
	assert_eq!(unsafecell.value, value);

	unsafe { *unsafecell.get() = 500; };

	assert_eq!(unsafecell.value, 500);
	assert_eq!(*value_ref, 500);
    }

    #[test]
    fn as_unsafe_call_test3() {
	let value: T = 68;
	let refcell: RefCell<T> = RefCell::<T>::new(value);
	let value_refmut: RefMut<T> = refcell.borrow_mut();

	let unsafecell: &UnsafeCell<T> = unsafe { refcell.as_unsafe_cell() };
	assert_eq!(unsafecell.value, value);

	unsafe { *unsafecell.get() = 500 };

	assert_eq!(unsafecell.value, 500);
	assert_eq!(*value_refmut, 500);
    }
}
