#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::UnsafeCell;

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

    // impl<T: ?Sized> UnsafeCell<T> {
    //     /// Gets a mutable pointer to the wrapped value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::UnsafeCell;
    //     ///
    //     /// let uc = UnsafeCell::new(5);
    //     ///
    //     /// let five = uc.get();
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn get(&self) -> *mut T {
    //         // FIXME(#23542) Replace with type ascription.
    //         #![allow(trivial_casts)]
    //         &self.value as *const T as *mut T
    //     }
    // }

    type T = i32;

    #[test]
    fn get_test1() {
	let value: T = 68;
	let unsafecell: UnsafeCell<T> = UnsafeCell::<T>::new(value);
	let get: *mut T = unsafecell.get();

	unsafe { *get = 500; }

	assert_eq!(unsafecell.value, 500);
    }
}
