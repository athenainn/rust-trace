#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::cell::Cell;

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

    type T = i32;

    #[test]
    fn set_test1() {
	let value: T = 68;
	let cell: Cell<T> = Cell::<T>::new(value);
	{
	    let result: T = cell.get();
	    assert_eq!(result, value);
	}

	cell.set(500 as T);

	{
	    let result: T = cell.get();
	    assert_eq!(result, 500);
	}
    }
}
