#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    use core::default::Default;

    // pub struct Cell<T> {
    //     value: UnsafeCell<T>,
    // }

    // impl<T:Copy> Cell<T> {
    //     /// Creates a new `Cell` containing the given value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     /// ```
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     pub fn new(value: T) -> Cell<T> {
    //         Cell {
    //             value: UnsafeCell::new(value),
    //         }
    //     }
    //
    //     /// Returns a copy of the contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     ///
    //     /// let five = c.get();
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn get(&self) -> T {
    //         unsafe{ *self.value.get() }
    //     }
    //
    //     /// Sets the contained value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     ///
    //     /// c.set(10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn set(&self, value: T) {
    //         unsafe {
    //             *self.value.get() = value;
    //         }
    //     }
    //
    //     /// Gets a reference to the underlying `UnsafeCell`.
    //     ///
    //     /// # Unsafety
    //     ///
    //     /// This function is `unsafe` because `UnsafeCell`'s field is public.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(core)]
    //     /// use std::cell::Cell;
    //     ///
    //     /// let c = Cell::new(5);
    //     ///
    //     /// let uc = unsafe { c.as_unsafe_cell() };
    //     /// ```
    //     #[inline]
    //     #[unstable(feature = "core")]
    //     pub unsafe fn as_unsafe_cell<'a>(&'a self) -> &'a UnsafeCell<T> {
    //         &self.value
    //     }
    // }

    // impl<T:Default + Copy> Default for Cell<T> {
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     #[inline]
    //     fn default() -> Cell<T> {
    //         Cell::new(Default::default())
    //     }
    // }

    type T = i32;

    #[test]
    fn default_test1() {
	let default: Cell<T> = Cell::<T>::default();

	assert_eq!(default.get(), 0);
    }

    #[test]
    fn default_test2() {
	let default: Cell<T> = Default::default();

	assert_eq!(default.get(), 0);
    }
}
