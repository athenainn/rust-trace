#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::atomic_store_relaxed;

    use core::cell::UnsafeCell;

    use std::sync::Arc;

    use std::thread;

    // pub fn atomic_store_relaxed<T>(dst: *mut T, val: T);

    struct A<T> {
	v: UnsafeCell<T>
    }

    unsafe impl Sync for A<T> {}

    impl<T> A<T> {
	fn new(v: T) -> A<T> {
	    A { v: UnsafeCell::<T>::new(v) }
	}
    }

    type T = usize;

    macro_rules! atomic_store_relaxed_test {
	($init:expr, $value:expr) => ({
	    let value: T = $init;
	    let a: A<T> = A::<T>::new(value);

	    let data: Arc<A<T>> = Arc::<A<T>>::new(a);
	    let clone: Arc<A<T>> = data.clone();

	    thread::spawn(move || {
		let dst: *mut T = clone.v.get();
		let val: T = $value;

		unsafe { atomic_store_relaxed::<T>(dst, val) };
	    });

	    thread::sleep_ms(10);

	    let ptr: *mut T = data.v.get();
	    assert_eq!(unsafe { *ptr }, $value);
	})
    }

    #[test]
    fn atomic_store_relaxed_test1() {
	atomic_store_relaxed_test!( 68, 500 );
    }
}
