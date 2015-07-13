#![feature(core, alloc, rc_unique, rc_weak)]
extern crate core;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use alloc::rc::Weak;

    use core::mem::drop;

    // pub struct Rc<T: ?Sized> {
    //     // FIXME #12808: strange names to try to avoid interfering with field
    //     // accesses of the contained type via Deref
    //     _ptr: NonZero<*mut RcBox<T>>,
    // }

    // impl<T: ?Sized> !marker::Send for Rc<T> {}
    // impl<T: ?Sized> !marker::Sync for Rc<T> {}

    // impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<Rc<U>> for Rc<T> {}

    // impl<T> Rc<T> {
    //     /// Constructs a new `Rc<T>`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new(value: T) -> Rc<T> {
    //         unsafe {
    //             Rc {
    //                 // there is an implicit weak pointer owned by all the strong
    //                 // pointers, which ensures that the weak destructor never frees
    //                 // the allocation while the strong destructor is running, even
    //                 // if the weak pointer is stored inside the strong one.
    //                 _ptr: NonZero::new(Box::into_raw(box RcBox {
    //                     strong: Cell::new(1),
    //                     weak: Cell::new(1),
    //                     value: value
    //                 })),
    //             }
    //         }
    //     }
    //
    //     /// Unwraps the contained value if the `Rc<T>` is unique.
    //     ///
    //     /// If the `Rc<T>` is not unique, an `Err` is returned with the same
    //     /// `Rc<T>`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_unique)]
    //     /// use std::rc::Rc;
    //     ///
    //     /// let x = Rc::new(3);
    //     /// assert_eq!(Rc::try_unwrap(x), Ok(3));
    //     ///
    //     /// let x = Rc::new(4);
    //     /// let _y = x.clone();
    //     /// assert_eq!(Rc::try_unwrap(x), Err(Rc::new(4)));
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "rc_unique")]
    //     pub fn try_unwrap(rc: Rc<T>) -> Result<T, Rc<T>> {
    //         if Rc::is_unique(&rc) {
    //             unsafe {
    //                 let val = ptr::read(&*rc); // copy the contained object
    //                 // destruct the box and skip our Drop
    //                 // we can ignore the refcounts because we know we're unique
    //                 deallocate(*rc._ptr as *mut u8, size_of::<RcBox<T>>(),
    //                             align_of::<RcBox<T>>());
    //                 forget(rc);
    //                 Ok(val)
    //             }
    //         } else {
    //             Err(rc)
    //         }
    //     }
    // }

    // impl<T: ?Sized> Drop for Rc<T> {
    //     /// Drops the `Rc<T>`.
    //     ///
    //     /// This will decrement the strong reference count. If the strong reference
    //     /// count becomes zero and the only other references are `Weak<T>` ones,
    //     /// `drop`s the inner value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::rc::Rc;
    //     ///
    //     /// {
    //     ///     let five = Rc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     ///     drop(five); // explicit drop
    //     /// }
    //     /// {
    //     ///     let five = Rc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     /// } // implicit drop
    //     /// ```
    //     fn drop(&mut self) {
    //         unsafe {
    //             let ptr = *self._ptr;
    //             if !(*(&ptr as *const _ as *const *const ())).is_null() &&
    //                ptr as *const () as usize != mem::POST_DROP_USIZE {
    //                 self.dec_strong();
    //                 if self.strong() == 0 {
    //                     // destroy the contained object
    //                     drop_in_place(&mut (*ptr).value);
    //
    //                     // remove the implicit "strong weak" pointer now that we've
    //                     // destroyed the contents.
    //                     self.dec_weak();
    //
    //                     if self.weak() == 0 {
    //                         deallocate(ptr as *mut u8,
    //                                    size_of_val(&*ptr),
    //                                    align_of_val(&*ptr))
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub struct Weak<T: ?Sized> {
    //     // FIXME #12808: strange names to try to avoid interfering with
    //     // field accesses of the contained type via Deref
    //     _ptr: NonZero<*mut RcBox<T>>,
    // }

    // impl<T: ?Sized> !marker::Send for Weak<T> {}
    // impl<T: ?Sized> !marker::Sync for Weak<T> {}

    // impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<Weak<U>> for Weak<T> {}

    // impl<T: ?Sized> Weak<T> {
    //
    //     /// Upgrades a weak reference to a strong reference.
    //     ///
    //     /// Upgrades the `Weak<T>` reference to an `Rc<T>`, if possible.
    //     ///
    //     /// Returns `None` if there were no strong references and the data was
    //     /// destroyed.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_weak)]
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     ///
    //     /// let weak_five = five.downgrade();
    //     ///
    //     /// let strong_five: Option<Rc<_>> = weak_five.upgrade();
    //     /// ```
    //     pub fn upgrade(&self) -> Option<Rc<T>> {
    //         if self.strong() == 0 {
    //             None
    //         } else {
    //             self.inc_strong();
    //             Some(Rc { _ptr: self._ptr })
    //         }
    //     }
    // }

    // impl<T: ?Sized> Drop for Weak<T> {
    //     /// Drops the `Weak<T>`.
    //     ///
    //     /// This will decrement the weak reference count.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_weak)]
    //     /// use std::rc::Rc;
    //     ///
    //     /// {
    //     ///     let five = Rc::new(5);
    //     ///     let weak_five = five.downgrade();
    //     ///
    //     ///     // stuff
    //     ///
    //     ///     drop(weak_five); // explicit drop
    //     /// }
    //     /// {
    //     ///     let five = Rc::new(5);
    //     ///     let weak_five = five.downgrade();
    //     ///
    //     ///     // stuff
    //     ///
    //     /// } // implicit drop
    //     /// ```
    //     fn drop(&mut self) {
    //         unsafe {
    //             let ptr = *self._ptr;
    //             if !(*(&ptr as *const _ as *const *const ())).is_null() &&
    //                ptr as *const () as usize != mem::POST_DROP_USIZE {
    //                 self.dec_weak();
    //                 // the weak count starts at 1, and will only go to zero if all
    //                 // the strong pointers have disappeared.
    //                 if self.weak() == 0 {
    //                     deallocate(ptr as *mut u8, size_of_val(&*ptr),
    //                                align_of_val(&*ptr))
    //                 }
    //             }
    //         }
    //     }
    // }

    type T = i32; // T: ?Sized

    #[test]
    fn get_mut_test1() {
	let value: T = 68;
	let mut rc: Rc<T> = Rc::<T>::new(value);
	assert_eq!(*rc, 68);

	{
	    let get_mut: Option<&mut T> = Rc::<T>::get_mut(&mut rc);
	    match get_mut {
		Some(v) => *v = 500,
		None => assert!(false)
	    }
	}

	assert_eq!(*rc, 500);
    }

    #[test]
    fn get_mut_test2() {
	let value: T = 68;
	let mut rc: Rc<T> = Rc::<T>::new(value);
	assert_eq!(*rc, 68);

	let clone: Rc<T> = rc.clone();

	let get_mut: Option<&mut T> = Rc::<T>::get_mut(&mut rc);
	match get_mut {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}

	drop(clone);
    }

    #[test]
    fn get_mut_test3() {
	let value: T = 68;
	let mut rc: Rc<T> = Rc::<T>::new(value);
	assert_eq!(*rc, 68);

	let weak: Weak<T> = rc.downgrade();

	let get_mut: Option<&mut T> = Rc::<T>::get_mut(&mut rc);
	match get_mut {
	    Some(_) => assert!(false),
	    None => assert!(true)
	}

	drop(weak);
    }
}
