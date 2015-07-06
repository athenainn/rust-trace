#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::atomic::AtomicUsize;
    use core::atomic::Ordering::{Relaxed, Release, Acquire, AcqRel, SeqCst};

    use std::sync::Arc;

    use std::thread;

    // pub struct AtomicUsize {
    //     v: UnsafeCell<usize>,
    // }

    // impl Default for AtomicUsize {
    //     fn default() -> Self {
    //         Self::new(Default::default())
    //     }
    // }

    // unsafe impl Sync for AtomicUsize {}

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

    // impl AtomicUsize {
    //     /// Creates a new `AtomicUsize`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::AtomicUsize;
    //     ///
    //     /// let atomic_forty_two = AtomicUsize::new(42);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub const fn new(v: usize) -> AtomicUsize {
    //         AtomicUsize { v: UnsafeCell::new(v) }
    //     }
    //
    //     /// Loads a value from the usize.
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
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let some_usize = AtomicUsize::new(5);
    //     ///
    //     /// assert_eq!(some_usize.load(Ordering::Relaxed), 5);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn load(&self, order: Ordering) -> usize {
    //         unsafe { atomic_load(self.v.get(), order) }
    //     }
    //
    //     /// Stores a value into the usize.
    //     ///
    //     /// `store` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let some_usize = AtomicUsize::new(5);
    //     ///
    //     /// some_usize.store(10, Ordering::Relaxed);
    //     /// assert_eq!(some_usize.load(Ordering::Relaxed), 10);
    //     /// ```
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if `order` is `Acquire` or `AcqRel`.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn store(&self, val: usize, order: Ordering) {
    //         unsafe { atomic_store(self.v.get(), val, order); }
    //     }
    //
    //     /// Stores a value into the usize, returning the old value.
    //     ///
    //     /// `swap` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let some_usize= AtomicUsize::new(5);
    //     ///
    //     /// assert_eq!(some_usize.swap(10, Ordering::Relaxed), 5);
    //     /// assert_eq!(some_usize.load(Ordering::Relaxed), 10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn swap(&self, val: usize, order: Ordering) -> usize {
    //         unsafe { atomic_swap(self.v.get(), val, order) }
    //     }
    //
    //     /// Stores a value into the `usize` if the current value is the same as the `current` value.
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
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let some_usize = AtomicUsize::new(5);
    //     ///
    //     /// assert_eq!(some_usize.compare_and_swap(5, 10, Ordering::Relaxed), 5);
    //     /// assert_eq!(some_usize.load(Ordering::Relaxed), 10);
    //     ///
    //     /// assert_eq!(some_usize.compare_and_swap(6, 12, Ordering::Relaxed), 10);
    //     /// assert_eq!(some_usize.load(Ordering::Relaxed), 10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn compare_and_swap(&self, current: usize, new: usize, order: Ordering) -> usize {
    //         unsafe { atomic_compare_and_swap(self.v.get(), current, new, order) }
    //     }
    //
    //     /// Add to the current usize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let foo = AtomicUsize::new(0);
    //     /// assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 10);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_add(&self, val: usize, order: Ordering) -> usize {
    //         unsafe { atomic_add(self.v.get(), val, order) }
    //     }
    //
    //     /// Subtract from the current usize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let foo = AtomicUsize::new(10);
    //     /// assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 10);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 0);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_sub(&self, val: usize, order: Ordering) -> usize {
    //         unsafe { atomic_sub(self.v.get(), val, order) }
    //     }
    //
    //     /// Bitwise and with the current usize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let foo = AtomicUsize::new(0b101101);
    //     /// assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 0b100001);
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_and(&self, val: usize, order: Ordering) -> usize {
    //         unsafe { atomic_and(self.v.get(), val, order) }
    //     }
    //
    //     /// Bitwise or with the current usize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let foo = AtomicUsize::new(0b101101);
    //     /// assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 0b111111);
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_or(&self, val: usize, order: Ordering) -> usize {
    //         unsafe { atomic_or(self.v.get(), val, order) }
    //     }
    //
    //     /// Bitwise xor with the current usize, returning the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicUsize, Ordering};
    //     ///
    //     /// let foo = AtomicUsize::new(0b101101);
    //     /// assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), 0b011110);
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_xor(&self, val: usize, order: Ordering) -> usize {
    //         unsafe { atomic_xor(self.v.get(), val, order) }
    //     }
    // }
    //
    // impl<T> AtomicPtr<T> {
    //     /// Creates a new `AtomicPtr`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::AtomicPtr;
    //     ///
    //     /// let ptr = &mut 5;
    //     /// let atomic_ptr  = AtomicPtr::new(ptr);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub const fn new(p: *mut T) -> AtomicPtr<T> {
    //         AtomicPtr { p: UnsafeCell::new(p) }
    //     }
    //
    //     /// Loads a value from the pointer.
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
    //     /// use std::sync::atomic::{AtomicPtr, Ordering};
    //     ///
    //     /// let ptr = &mut 5;
    //     /// let some_ptr  = AtomicPtr::new(ptr);
    //     ///
    //     /// let value = some_ptr.load(Ordering::Relaxed);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn load(&self, order: Ordering) -> *mut T {
    //         unsafe {
    //             atomic_load(self.p.get() as *mut usize, order) as *mut T
    //         }
    //     }
    //
    //     /// Stores a value into the pointer.
    //     ///
    //     /// `store` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicPtr, Ordering};
    //     ///
    //     /// let ptr = &mut 5;
    //     /// let some_ptr  = AtomicPtr::new(ptr);
    //     ///
    //     /// let other_ptr = &mut 10;
    //     ///
    //     /// some_ptr.store(other_ptr, Ordering::Relaxed);
    //     /// ```
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if `order` is `Acquire` or `AcqRel`.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn store(&self, ptr: *mut T, order: Ordering) {
    //         unsafe { atomic_store(self.p.get() as *mut usize, ptr as usize, order); }
    //     }
    //
    //     /// Stores a value into the pointer, returning the old value.
    //     ///
    //     /// `swap` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicPtr, Ordering};
    //     ///
    //     /// let ptr = &mut 5;
    //     /// let some_ptr  = AtomicPtr::new(ptr);
    //     ///
    //     /// let other_ptr = &mut 10;
    //     ///
    //     /// let value = some_ptr.swap(other_ptr, Ordering::Relaxed);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T {
    //         unsafe { atomic_swap(self.p.get() as *mut usize, ptr as usize, order) as *mut T }
    //     }
    //
    //     /// Stores a value into the pointer if the current value is the same as the `current` value.
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
    //     /// use std::sync::atomic::{AtomicPtr, Ordering};
    //     ///
    //     /// let ptr = &mut 5;
    //     /// let some_ptr  = AtomicPtr::new(ptr);
    //     ///
    //     /// let other_ptr   = &mut 10;
    //     /// let another_ptr = &mut 10;
    //     ///
    //     /// let value = some_ptr.compare_and_swap(other_ptr, another_ptr, Ordering::Relaxed);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
    //         unsafe {
    //             atomic_compare_and_swap(self.p.get() as *mut usize, current as usize,
    //                                     new as usize, order) as *mut T
    //         }
    //     }
    // }

    // pub const ATOMIC_USIZE_INIT: AtomicUsize = AtomicUsize::new(0);

    macro_rules! swap_test {
	($($order:ident)*) => ($({
	    let atomicusize: AtomicUsize = AtomicUsize::default();

	    let result: usize = atomicusize.load(Relaxed);
	    assert_eq!(result, 0);

	    let data: Arc<AtomicUsize> = Arc::<AtomicUsize>::new(atomicusize);
	    let clone: Arc<AtomicUsize> = data.clone();

	    let result: usize = clone.load(Relaxed);
	    assert_eq!(result, 0);

	    thread::spawn(move || {
		let val: usize = 68;

		let result: usize = clone.load(Relaxed);
		assert_eq!(result, 0);

		let old: usize = clone.swap(val, $order);
		assert_eq!(old, 0);
	    });

	    thread::sleep_ms(10);

	    let result: usize = data.load(Relaxed);
	    assert_eq!(result, 68);
	})*)
    }


    #[test]
    fn swap_test1() {
	swap_test!( Relaxed Release Acquire AcqRel SeqCst );
    }
}
