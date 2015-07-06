#![feature(core)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::atomic::AtomicBool;
    use core::atomic::Ordering::{Relaxed, Release, Acquire, AcqRel, SeqCst};

    use std::sync::Arc;

    use std::thread;

    // pub struct AtomicBool {
    //     v: UnsafeCell<usize>,
    // }

    // impl Default for AtomicBool {
    //     fn default() -> Self {
    //         Self::new(Default::default())
    //     }
    // }

    // unsafe impl Sync for AtomicBool {}

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

    // impl AtomicBool {
    //     /// Creates a new `AtomicBool`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::AtomicBool;
    //     ///
    //     /// let atomic_true  = AtomicBool::new(true);
    //     /// let atomic_false = AtomicBool::new(false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub const fn new(v: bool) -> AtomicBool {
    //         AtomicBool { v: UnsafeCell::new(-(v as isize) as usize) }
    //     }
    //
    //     /// Loads a value from the bool.
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
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let some_bool = AtomicBool::new(true);
    //     ///
    //     /// assert_eq!(some_bool.load(Ordering::Relaxed), true);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn load(&self, order: Ordering) -> bool {
    //         unsafe { atomic_load(self.v.get(), order) > 0 }
    //     }
    //
    //     /// Stores a value into the bool.
    //     ///
    //     /// `store` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let some_bool = AtomicBool::new(true);
    //     ///
    //     /// some_bool.store(false, Ordering::Relaxed);
    //     /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    //     /// ```
    //     ///
    //     /// # Panics
    //     ///
    //     /// Panics if `order` is `Acquire` or `AcqRel`.
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn store(&self, val: bool, order: Ordering) {
    //         let val = if val { UINT_TRUE } else { 0 };
    //
    //         unsafe { atomic_store(self.v.get(), val, order); }
    //     }
    //
    //     /// Stores a value into the bool, returning the old value.
    //     ///
    //     /// `swap` takes an `Ordering` argument which describes the memory ordering of this operation.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let some_bool = AtomicBool::new(true);
    //     ///
    //     /// assert_eq!(some_bool.swap(false, Ordering::Relaxed), true);
    //     /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn swap(&self, val: bool, order: Ordering) -> bool {
    //         let val = if val { UINT_TRUE } else { 0 };
    // 
    //         unsafe { atomic_swap(self.v.get(), val, order) > 0 }
    //     }
    //
    //     /// Stores a value into the `bool` if the current value is the same as the `current` value.
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
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let some_bool = AtomicBool::new(true);
    //     ///
    //     /// assert_eq!(some_bool.compare_and_swap(true, false, Ordering::Relaxed), true);
    //     /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    //     ///
    //     /// assert_eq!(some_bool.compare_and_swap(true, true, Ordering::Relaxed), false);
    //     /// assert_eq!(some_bool.load(Ordering::Relaxed), false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn compare_and_swap(&self, current: bool, new: bool, order: Ordering) -> bool {
    //         let current = if current { UINT_TRUE } else { 0 };
    //         let new = if new { UINT_TRUE } else { 0 };
    // 
    //         unsafe { atomic_compare_and_swap(self.v.get(), current, new, order) > 0 }
    //     }
    //
    //     /// Logical "and" with a boolean value.
    //     ///
    //     /// Performs a logical "and" operation on the current value and the argument `val`, and sets
    //     /// the new value to the result.
    //     ///
    //     /// Returns the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_and(false, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), false);
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_and(true, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), true);
    //     ///
    //     /// let foo = AtomicBool::new(false);
    //     /// assert_eq!(foo.fetch_and(false, Ordering::SeqCst), false);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_and(&self, val: bool, order: Ordering) -> bool {
    //         let val = if val { UINT_TRUE } else { 0 };
    //
    //         unsafe { atomic_and(self.v.get(), val, order) > 0 }
    //     }
    //
    //     /// Logical "nand" with a boolean value.
    //     ///
    //     /// Performs a logical "nand" operation on the current value and the argument `val`, and sets
    //     /// the new value to the result.
    //     ///
    //     /// Returns the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_nand(false, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), true);
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_nand(true, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst) as usize, 0);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), false);
    //     ///
    //     /// let foo = AtomicBool::new(false);
    //     /// assert_eq!(foo.fetch_nand(false, Ordering::SeqCst), false);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), true);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_nand(&self, val: bool, order: Ordering) -> bool {
    //         let val = if val { UINT_TRUE } else { 0 };
    // 
    //         unsafe { atomic_nand(self.v.get(), val, order) > 0 }
    //     }
    //
    //     /// Logical "or" with a boolean value.
    //     ///
    //     /// Performs a logical "or" operation on the current value and the argument `val`, and sets the
    //     /// new value to the result.
    //     ///
    //     /// Returns the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_or(false, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), true);
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_or(true, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), true);
    //     ///
    //     /// let foo = AtomicBool::new(false);
    //     /// assert_eq!(foo.fetch_or(false, Ordering::SeqCst), false);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_or(&self, val: bool, order: Ordering) -> bool {
    //         let val = if val { UINT_TRUE } else { 0 };
    // 
    //         unsafe { atomic_or(self.v.get(), val, order) > 0 }
    //     }
    //
    //     /// Logical "xor" with a boolean value.
    //     ///
    //     /// Performs a logical "xor" operation on the current value and the argument `val`, and sets
    //     /// the new value to the result.
    //     ///
    //     /// Returns the previous value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::atomic::{AtomicBool, Ordering};
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_xor(false, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), true);
    //     ///
    //     /// let foo = AtomicBool::new(true);
    //     /// assert_eq!(foo.fetch_xor(true, Ordering::SeqCst), true);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), false);
    //     ///
    //     /// let foo = AtomicBool::new(false);
    //     /// assert_eq!(foo.fetch_xor(false, Ordering::SeqCst), false);
    //     /// assert_eq!(foo.load(Ordering::SeqCst), false);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
    //         let val = if val { UINT_TRUE } else { 0 };
    //
    //         unsafe { atomic_xor(self.v.get(), val, order) > 0 }
    //     }
    // }

    // const UINT_TRUE: usize = !0;

    macro_rules! fetch_and_test {
	($init:expr, $val:expr, $order:ident, $result:expr) => ({
	    let atomicbool: AtomicBool = AtomicBool::new($init);

	    let result: bool = atomicbool.load(Relaxed);
	    assert_eq!(result, $init);

	    let data: Arc<AtomicBool> = Arc::<AtomicBool>::new(atomicbool);
	    let clone: Arc<AtomicBool> = data.clone();

	    let result: bool = clone.load(Relaxed);
	    assert_eq!(result, $init);

	    thread::spawn(move || {
		let val: bool = $val;

		let result: bool = clone.load(Relaxed);
		assert_eq!(result, $init);

		let result: bool = clone.fetch_and(val, $order);
		assert_eq!(result, $init);
	    });

	    thread::sleep_ms(10);

	    let result: bool = data.load(Relaxed);
	    assert_eq!(result, $result);
	})
    }

    #[test]
    fn fetch_and_test1() {
	fetch_and_test!( false, false, Relaxed, false );
	fetch_and_test!( false, false, Release, false );
	fetch_and_test!( false, false, Acquire, false );
	fetch_and_test!( false, false, AcqRel, false );
	fetch_and_test!( false, false, SeqCst, false );
    }

    #[test]
    fn fetch_and_test2() {
	fetch_and_test!( false, true, Relaxed, false );
	fetch_and_test!( false, true, Release, false );
	fetch_and_test!( false, true, Acquire, false );
	fetch_and_test!( false, true, AcqRel, false );
	fetch_and_test!( false, true, SeqCst, false );
    }

    #[test]
    fn fetch_and_test3() {
	fetch_and_test!( true, false, Relaxed, false );
	fetch_and_test!( true, false, Release, false );
	fetch_and_test!( true, false, Acquire, false );
	fetch_and_test!( true, false, AcqRel, false );
	fetch_and_test!( true, false, SeqCst, false );
    }

    #[test]
    fn fetch_and_test4() {
	fetch_and_test!( true, true, Relaxed, true );
	fetch_and_test!( true, true, Release, true );
	fetch_and_test!( true, true, Acquire, true );
	fetch_and_test!( true, true, AcqRel, true );
	fetch_and_test!( true, true, SeqCst, true );
    }
}
