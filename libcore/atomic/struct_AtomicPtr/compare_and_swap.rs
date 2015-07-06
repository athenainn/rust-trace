#![feature(core, const_fn)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::atomic::AtomicPtr;
    use core::atomic::Ordering::{Relaxed, Release, Acquire, AcqRel, SeqCst};

    use core::marker::Send;

    use core::ptr::null_mut;

    use std::sync::Arc;

    use std::thread;

    // pub struct AtomicPtr<T> {
    //     p: UnsafeCell<*mut T>,
    // }

    // impl<T> Default for AtomicPtr<T> {
    //     fn default() -> AtomicPtr<T> {
    //         AtomicPtr::new(::ptr::null_mut())
    //     }
    // }

    // unsafe impl<T> Sync for AtomicPtr<T> {}

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

    struct A<T> {
	ptr: AtomicPtr<T>
    }

    impl<T> A<T> {
	const fn new(p: *mut T) -> Self {
	    A { ptr: AtomicPtr::<T>::new(p) }
	}
    }

    unsafe impl<T> Send for A<T> {}

    type T = i32;

    macro_rules! compare_and_swap_test {
	($init:expr, $current:expr, $new:expr, $order:ident, $result:expr) => ({
	    let a: A<T> = A::<T>::new($init);
	    let data: Arc<A<T>> = Arc::<A<T>>::new(a);
	    let clone: Arc<A<T>> = data.clone();

	    let current_clone: Arc<A<T>> = $current.clone();
	    let new_clone: Arc<A<T>> = $new.clone();
	    let init_clone: Arc<A<T>> = Arc::<A<T>>::new(A::<T>::new($init)).clone();

	    thread::spawn(move || {
		let current: *mut T = current_clone.ptr.load(Relaxed);
		let new: *mut T = new_clone.ptr.load(Relaxed);

		let result: *mut T = clone.ptr.compare_and_swap(current, new, $order);
		assert_eq!(result, init_clone.ptr.load(Relaxed));
	    });

	    thread::sleep_ms(10);

	    let result: *mut T = data.ptr.load(Relaxed);
	    assert_eq!(result, $result.ptr.load(Relaxed));
	})
    }

    #[test]
    fn compare_and_swap_test1() {
	let p: *mut T = &mut 68;
	let n: *mut T = null_mut::<T>();

	let ap: A<T> = A::<T>::new(p);
	let an: A<T> = A::<T>::new(n);

	let ap_data: Arc<A<T>> = Arc::<A<T>>::new(ap);
	let an_data: Arc<A<T>> = Arc::<A<T>>::new(an);

	compare_and_swap_test!( p, ap_data, an_data, Relaxed, an_data );
	compare_and_swap_test!( p, ap_data, an_data, Release, an_data );
	compare_and_swap_test!( p, ap_data, an_data, Acquire, an_data );
	compare_and_swap_test!( p, ap_data, an_data, AcqRel, an_data );
	compare_and_swap_test!( p, ap_data, an_data, SeqCst, an_data );
    }

    #[test]
    fn compare_and_swap_test2() {
	let p: *mut T = &mut 68;
	let n: *mut T = null_mut::<T>();

	let ap: A<T> = A::<T>::new(p);
	let an: A<T> = A::<T>::new(n);

	let ap_data: Arc<A<T>> = Arc::<A<T>>::new(ap);
	let an_data: Arc<A<T>> = Arc::<A<T>>::new(an);

	compare_and_swap_test!( p, an_data, an_data, Relaxed, ap_data );
	compare_and_swap_test!( p, an_data, an_data, Release, ap_data );
	compare_and_swap_test!( p, an_data, an_data, Acquire, ap_data );
	compare_and_swap_test!( p, an_data, an_data, AcqRel, ap_data );
	compare_and_swap_test!( p, an_data, an_data, SeqCst, ap_data );
    }

    #[test]
    fn compare_and_swap_test3() {
	let p: *mut T = &mut 68;
	let n: *mut T = null_mut::<T>();

	let ap: A<T> = A::<T>::new(p);
	let an: A<T> = A::<T>::new(n);

	let ap_data: Arc<A<T>> = Arc::<A<T>>::new(ap);
	let an_data: Arc<A<T>> = Arc::<A<T>>::new(an);

	compare_and_swap_test!( n, ap_data, ap_data, Relaxed, an_data );
	compare_and_swap_test!( n, ap_data, ap_data, Release, an_data );
	compare_and_swap_test!( n, ap_data, ap_data, Acquire, an_data );
	compare_and_swap_test!( n, ap_data, ap_data, AcqRel, an_data );
	compare_and_swap_test!( n, ap_data, ap_data, SeqCst, an_data );
    }

    #[test]
    fn compare_and_swap_test4() {
	let p: *mut T = &mut 68;
	let n: *mut T = null_mut::<T>();

	let ap: A<T> = A::<T>::new(p);
	let an: A<T> = A::<T>::new(n);

	let ap_data: Arc<A<T>> = Arc::<A<T>>::new(ap);
	let an_data: Arc<A<T>> = Arc::<A<T>>::new(an);

	compare_and_swap_test!( n, an_data, ap_data, Relaxed, ap_data );
	compare_and_swap_test!( n, an_data, ap_data, Release, ap_data );
	compare_and_swap_test!( n, an_data, ap_data, Acquire, ap_data );
	compare_and_swap_test!( n, an_data, ap_data, AcqRel, ap_data );
	compare_and_swap_test!( n, an_data, ap_data, SeqCst, ap_data );
    }
}
