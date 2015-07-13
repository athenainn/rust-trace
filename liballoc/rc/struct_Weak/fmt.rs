#![feature(alloc, rc_weak, rc_counts)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use alloc::rc::Weak;

    // pub struct Rc<T: ?Sized> {
    //     // FIXME #12808: strange names to try to avoid interfering with field
    //     // accesses of the contained type via Deref
    //     _ptr: NonZero<*mut RcBox<T>>,
    // }

    // impl<T: ?Sized> !marker::Send for Rc<T> {}
    // impl<T: ?Sized> !marker::Sync for Rc<T> {}

    // impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<Rc<U>> for Rc<T> {}

    // impl<T: ?Sized> Rc<T> {
    //     /// Downgrades the `Rc<T>` to a `Weak<T>` reference.
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
    //     /// ```
    //     #[unstable(feature = "rc_weak",
    //                reason = "Weak pointers may not belong in this module")]
    //     pub fn downgrade(&self) -> Weak<T> {
    //         self.inc_weak();
    //         Weak { _ptr: self._ptr }
    //     }
    //
    //     /// Get the number of weak references to this value.
    //     #[inline]
    //     #[unstable(feature = "rc_counts")]
    //     pub fn weak_count(this: &Rc<T>) -> usize { this.weak() - 1 }
    //
    //     /// Get the number of strong references to this value.
    //     #[inline]
    //     #[unstable(feature = "rc_counts")]
    //     pub fn strong_count(this: &Rc<T>) -> usize { this.strong() }
    //
    //     /// Returns true if there are no other `Rc` or `Weak<T>` values that share
    //     /// the same inner value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_unique)]
    //     /// use std::rc::Rc;
    //     ///
    //     /// let five = Rc::new(5);
    //     ///
    //     /// assert!(Rc::is_unique(&five));
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "rc_unique")]
    //     pub fn is_unique(rc: &Rc<T>) -> bool {
    //         Rc::weak_count(rc) == 0 && Rc::strong_count(rc) == 1
    //     }
    //
    //     /// Returns a mutable reference to the contained value if the `Rc<T>` is
    //     /// unique.
    //     ///
    //     /// Returns `None` if the `Rc<T>` is not unique.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(rc_unique)]
    //     /// use std::rc::Rc;
    //     ///
    //     //     /// let mut x = Rc::new(3);
    //     /// *Rc::get_mut(&mut x).unwrap() = 4;
    //     /// assert_eq!(*x, 4);
    //     ///
    //     /// let _y = x.clone();
    //     /// assert!(Rc::get_mut(&mut x).is_none());
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "rc_unique")]
    //     pub fn get_mut(rc: &mut Rc<T>) -> Option<&mut T> {
    //         if Rc::is_unique(rc) {
    //             let inner = unsafe { &mut **rc._ptr };
    //             Some(&mut inner.value)
    //         } else {
    //             None
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

    // impl<T: ?Sized+fmt::Debug> fmt::Debug for Weak<T> {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "(Weak)")
    //     }
    // }

    type T = i32; // T: ?Sized + fmt::Debug

    #[test]
    fn clone_test1() {
	let value: T = 68;
	let rc: Rc<T> = Rc::<T>::new(value);
	assert_eq!(*rc, 68);

	let weak: Weak<T> = rc.downgrade();
	let weak_count: usize = Rc::<T>::weak_count(&rc);
	assert_eq!(weak_count, 1);

	{
	    let weak: Weak<T> = weak.clone();
	    let weak_count: usize = Rc::<T>::weak_count(&rc);
	    assert_eq!(weak_count, 2);

	    let result: Option<Rc<T>> = weak.upgrade();

	    match result {
		Some(rc) => assert_eq!(*rc, 68),
		None => assert!(false)
	    }
	}
    }
}
