#![feature(alloc)]
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::arc::Arc;

    // pub struct Arc<T: ?Sized> {
    //     // FIXME #12808: strange name to try to avoid interfering with
    //     // field accesses of the contained type via Deref
    //     _ptr: NonZero<*mut ArcInner<T>>,
    // }

    // unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> { }
    // unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> { }

    // impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Arc<U>> for Arc<T> {}

    // impl<T> Arc<T> {
    //     /// Constructs a new `Arc<T>`.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     /// ```
    //     #[inline]
    //     #[stable(feature = "rust1", since = "1.0.0")]
    //     pub fn new(data: T) -> Arc<T> {
    //         // Start the weak pointer count as 1 which is the weak pointer that's
    //         // held by all the strong pointers (kinda), see std/rc.rs for more info
    //         let x: Box<_> = box ArcInner {
    //             strong: atomic::AtomicUsize::new(1),
    //             weak: atomic::AtomicUsize::new(1),
    //             data: data,
    //         };
    //         Arc { _ptr: unsafe { NonZero::new(mem::transmute(x)) } }
    //     }
    // }

    // impl<T: ?Sized> Arc<T> {
    //     /// Downgrades the `Arc<T>` to a `Weak<T>` reference.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// # #![feature(arc_weak)]
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// let weak_five = five.downgrade();
    //     /// ```
    //     #[unstable(feature = "arc_weak",
    //                reason = "Weak pointers may not belong in this module.")]
    //     pub fn downgrade(&self) -> Weak<T> {
    //         loop {
    //             // This Relaxed is OK because we're checking the value in the CAS
    //             // below.
    //             let cur = self.inner().weak.load(Relaxed);
    //
    //             // check if the weak counter is currently "locked"; if so, spin.
    //             if cur == usize::MAX { continue }
    //
    //             // NOTE: this code currently ignores the possibility of overflow
    //             // into usize::MAX; in general both Rc and Arc need to be adjusted
    //             // to deal with overflow.
    //
    //             // Unlike with Clone(), we need this to be an Acquire read to
    //             // synchronize with the write coming from `is_unique`, so that the
    //             // events prior to that write happen before this read.
    //             if self.inner().weak.compare_and_swap(cur, cur + 1, Acquire) == cur {
    //                 return Weak { _ptr: self._ptr }
    //             }
    //         }
    //     }
    //
    //     /// Get the number of weak references to this value.
    //     #[inline]
    //     #[unstable(feature = "arc_counts")]
    //     pub fn weak_count(this: &Arc<T>) -> usize {
    //         this.inner().weak.load(SeqCst) - 1
    //     }
    //
    //     /// Get the number of strong references to this value.
    //     #[inline]
    //     #[unstable(feature = "arc_counts")]
    //     pub fn strong_count(this: &Arc<T>) -> usize {
    //         this.inner().strong.load(SeqCst)
    //     }
    //
    //     #[inline]
    //     fn inner(&self) -> &ArcInner<T> {
    //         // This unsafety is ok because while this arc is alive we're guaranteed
    //         // that the inner pointer is valid. Furthermore, we know that the
    //         // `ArcInner` structure itself is `Sync` because the inner data is
    //         // `Sync` as well, so we're ok loaning out an immutable pointer to these
    //         // contents.
    //         unsafe { &**self._ptr }
    //     }
    //
    //     // Non-inlined part of `drop`.
    //     #[inline(never)]
    //     unsafe fn drop_slow(&mut self) {
    //         let ptr = *self._ptr;
    //
    //         // Destroy the data at this time, even though we may not free the box
    //         // allocation itself (there may still be weak pointers lying around).
    //         drop_in_place(&mut (*ptr).data);
    //
    //         if self.inner().weak.fetch_sub(1, Release) == 1 {
    //             atomic::fence(Acquire);
    //             deallocate(ptr as *mut u8, size_of_val(&*ptr), align_of_val(&*ptr))
    //         }
    //     }
    // }

    // impl<T: ?Sized> Drop for Arc<T> {
    //     /// Drops the `Arc<T>`.
    //     ///
    //     /// This will decrement the strong reference count. If the strong reference
    //     /// count becomes zero and the only other references are `Weak<T>` ones,
    //     /// `drop`s the inner value.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// {
    //     ///     let five = Arc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     ///     drop(five); // explicit drop
    //     /// }
    //     /// {
    //     ///     let five = Arc::new(5);
    //     ///
    //     ///     // stuff
    //     ///
    //     /// } // implicit drop
    //     /// ```
    //     #[inline]
    //     fn drop(&mut self) {
    //         // This structure has #[unsafe_no_drop_flag], so this drop glue may run
    //         // more than once (but it is guaranteed to be zeroed after the first if
    //         // it's run more than once)
    //         let ptr = *self._ptr;
    //         // if ptr.is_null() { return }
    //         if ptr as *mut u8 as usize == 0 || ptr as *mut u8 as usize == mem::POST_DROP_USIZE {
    //             return
    //         }
    //
    //         // Because `fetch_sub` is already atomic, we do not need to synchronize
    //         // with other threads unless we are going to delete the object. This
    //         // same logic applies to the below `fetch_sub` to the `weak` count.
    //         if self.inner().strong.fetch_sub(1, Release) != 1 { return }
    //
    //         // This fence is needed to prevent reordering of use of the data and
    //         // deletion of the data.  Because it is marked `Release`, the decreasing
    //         // of the reference count synchronizes with this `Acquire` fence. This
    //         // means that use of the data happens before decreasing the reference
    //         // count, which happens before this fence, which happens before the
    //         // deletion of the data.
    //         //
    //         // As explained in the [Boost documentation][1],
    //         //
    //         // > It is important to enforce any possible access to the object in one
    //         // > thread (through an existing reference) to *happen before* deleting
    //         // > the object in a different thread. This is achieved by a "release"
    //         // > operation after dropping a reference (any access to the object
    //         // > through this reference must obviously happened before), and an
    //         // > "acquire" operation before deleting the object.
    //         //
    //         // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
    //         atomic::fence(Acquire);
    //
    //         unsafe {
    //             self.drop_slow()
    //         }
    //     }
    // }

    // impl<T: ?Sized + PartialOrd> PartialOrd for Arc<T> {
    //     /// Partial comparison for two `Arc<T>`s.
    //     ///
    //     /// The two are compared by calling `partial_cmp()` on their inner values.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// five.partial_cmp(&Arc::new(5));
    //     /// ```
    //     fn partial_cmp(&self, other: &Arc<T>) -> Option<Ordering> {
    //         (**self).partial_cmp(&**other)
    //     }
    //
    //     /// Less-than comparison for two `Arc<T>`s.
    //     ///
    //     /// The two are compared by calling `<` on their inner values.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// five < Arc::new(5);
    //     /// ```
    //     fn lt(&self, other: &Arc<T>) -> bool { *(*self) < *(*other) }
    //
    //     /// 'Less-than or equal to' comparison for two `Arc<T>`s.
    //     ///
    //     /// The two are compared by calling `<=` on their inner values.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// five <= Arc::new(5);
    //     /// ```
    //     fn le(&self, other: &Arc<T>) -> bool { *(*self) <= *(*other) }
    //
    //     /// Greater-than comparison for two `Arc<T>`s.
    //     ///
    //     /// The two are compared by calling `>` on their inner values.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// five > Arc::new(5);
    //     /// ```
    //     fn gt(&self, other: &Arc<T>) -> bool { *(*self) > *(*other) }
    //
    //     /// 'Greater-than or equal to' comparison for two `Arc<T>`s.
    //     ///
    //     /// The two are compared by calling `>=` on their inner values.
    //     ///
    //     /// # Examples
    //     ///
    //     /// ```
    //     /// use std::sync::Arc;
    //     ///
    //     /// let five = Arc::new(5);
    //     ///
    //     /// five >= Arc::new(5);
    //     /// ```
    //     fn ge(&self, other: &Arc<T>) -> bool { *(*self) >= *(*other) }
    // }

    type T = i32; // T: ?Sized + PartialOrd

    macro_rules! lt_test {
	($T:ty, $value:expr, $other:expr, $result:expr) => ({
	    let arc: Arc<$T> = Arc::<$T>::new($value);
	    let other: Arc<$T> = Arc::<$T>::new($other);

	    assert_eq!(arc.lt(&other), $result);
	    assert_eq!(arc < other, $result);
	})
    }

    #[test]
    fn lt_test1() {
	lt_test!( T, 68, 68, false );
	lt_test!( T, 68, 500, true );
	lt_test!( T, 500, 68, false );
	lt_test!( T, 500, 500, false );
    }
}
