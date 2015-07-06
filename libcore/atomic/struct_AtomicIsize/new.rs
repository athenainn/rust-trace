#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::atomic::AtomicIsize;
    use core::atomic::Ordering::Relaxed;

    // pub struct AtomicIsize {
    //     v: UnsafeCell<isize>,
    // }

    // impl Default for AtomicIsize {
    //     fn default() -> Self {
    //         Self::new(Default::default())
    //     }
    // }

    // unsafe impl Sync for AtomicIsize {}

    // #[derive(Copy, Clone)]
    // pub enum Ordering {
    //     /// No ordering constraints, only atomic operations.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Relaxed,
    //     /// When coupled with a store, all previous writes become visible
    //     /// to another thread that performs a load with `Acquire` ordering
    //     /// on the same value.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Release,
    //     /// When coupled with a load, all subsequent loads will see data
    //     /// written before a store with `Release` ordering on the same value
    //     /// in another thread.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     Acquire,
    //     /// When coupled with a load, uses `Acquire` ordering, and with a store
    //     /// `Release` ordering.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     AcqRel,
    //     /// Like `AcqRel` with the additional guarantee that all threads see all
    //     /// sequentially consistent operations in the same order.
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     SeqCst,
    // }

    // impl AtomicIsize {
    //     /// Creates a new `AtomicIsize`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::AtomicIsize;
    //     ///
    //     /// let atomic_forty_two  = AtomicIsize::new(42);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub const fn new(v: isize) -> AtomicIsize {
    //         AtomicIsize {v: UnsafeCell::new(v)}
    //     }
    //
    //     /// Loads a value from the isize.
    //     ///
    //     /// `load` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if `order` is `Release` or `AcqRel`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let some_isize = AtomicIsize::new(5);
    //     ///
    //     /// assert_eq!(some_isize.load(Ordering::Relaxed), 5);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn load(&self, order: Ordering) -> isize {
    //         unsafe { atomic_load(self.v.get(), order) }
    //     }
    //
    //     /// Stores a value into the isize.
    //     ///
    //     /// `store` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let some_isize = AtomicIsize::new(5);
    //     ///
    //     /// some_isize.store(10, Ordering::Relaxed);
    //     /// assert_eq!(some_isize.load(Ordering::Relaxed), 10);
    //     /// ```
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if `order` is `Acquire` or `AcqRel`.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn store(&self, val: isize, order: Ordering) {
    //         unsafe { atomic_store(self.v.get(), val, order); }
    //     }
    //
    //     /// Stores a value into the isize, returning the old value.
    //     ///
    //     /// `swap` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let some_isize = AtomicIsize::new(5);
    //     ///
    //     /// assert_eq!(some_isize.swap(10, Ordering::Relaxed), 5);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn swap(&self, val: isize, order: Ordering) -> isize {
    //         unsafe { atomic_swap(self.v.get(), val, order) }
    //     }
    //
    //     /// Stores a value into the `isize` if the current value is the same as the `current` value.
    //     ///
    //     /// The return value is always the previous value. If it is equal to `current`, then the value
    //     /// was updated.
    //     ///
    //     /// `compare_and_swap` also takes an `Ordering` argument which describes the memory ordering of
    //     /// this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let some_isize = AtomicIsize::new(5);
    //     ///
    //     /// assert_eq!(some_isize.compare_and_swap(5, 10, Ordering::Relaxed), 5);
    //     /// assert_eq!(some_isize.load(Ordering::Relaxed), 10);
    //     ///
    //     /// assert_eq!(some_isize.compare_and_swap(6, 12, Ordering::Relaxed), 10);
    //     /// assert_eq!(some_isize.load(Ordering::Relaxed), 10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn compare_and_swap(&self, current: isize, new: isize, order: Ordering) -> isize {
    //         unsafe { atomic_compare_and_swap(self.v.get(), current, new, order) }
    //     }
    //
    //     /// Add an isize to the current value, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let foo = AtomicIsize::new(0);
    //     /// assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_add(&self, val: isize, order: Ordering) -> isize {
    //         unsafe { atomic_add(self.v.get(), val, order) }
    //     }
    //
    //     /// Subtract an isize from the current value, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let foo = AtomicIsize::new(0);
    //     /// assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 0);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), -10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_sub(&self, val: isize, order: Ordering) -> isize {
    //         unsafe { atomic_sub(self.v.get(), val, order) }
    //     }
    //
    //     /// Bitwise and with the current isize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let foo = AtomicIsize::new(0b101101);
    //     /// assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_and(&self, val: isize, order: Ordering) -> isize {
    //         unsafe { atomic_and(self.v.get(), val, order) }
    //     }
    //
    //     /// Bitwise or with the current isize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let foo = AtomicIsize::new(0b101101);
    //     /// assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_or(&self, val: isize, order: Ordering) -> isize {
    //         unsafe { atomic_or(self.v.get(), val, order) }
    //     }
    //
    //     /// Bitwise xor with the current isize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicIsize, Ordering};
    //     ///
    //     /// let foo = AtomicIsize::new(0b101101);
    //     /// assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_xor(&self, val: isize, order: Ordering) -> isize {
    //         unsafe { atomic_xor(self.v.get(), val, order) }
    //     }
    // }

    // pub const ATOMIC_ISIZE_INIT: AtomicIsize = AtomicIsize::new(0);

    #[test]
    fn new_test1() {
	let atomicisize: AtomicIsize = AtomicIsize::new(0);
	let result: isize = atomicisize.load(Relaxed);

	assert_eq!(result, 0);
    }

    #[test]
    fn new_test2() {
	let atomicisize: AtomicIsize = AtomicIsize::new(68);
	let result: isize = atomicisize.load(Relaxed);

	assert_eq!(result, 68);
    }

    #[test]
    fn new_test3() {
	let atomicisize: AtomicIsize = AtomicIsize::new(-68);
	let result: isize = atomicisize.load(Relaxed);

	assert_eq!(result, -68);
    }
}
