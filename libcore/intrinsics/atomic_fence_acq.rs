#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::atomic_fence_acq;
    use core::intrinsics::atomic_fence_rel;

    use core::cell::UnsafeCell;

    use std::sync::Arc;

    use std::thread;

    // pub fn atomic_fence_acq();

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

    macro_rules! atomic_fence_acq_test {
	($init:expr, $value:expr, $result:expr) => ({
	    let value: T = $init;
	    let a: A<T> = A::<T>::new(value);

	    let data: Arc<A<T>> = Arc::<A<T>>::new(a);
	    let clone: Arc<A<T>> = data.clone();

	    thread::spawn(move || {
		let ptr: *mut T = clone.v.get();

		unsafe { *ptr = $value; }
		unsafe { atomic_fence_rel() };
	    });

	    thread::sleep_ms(10);

	    unsafe { atomic_fence_acq() };

	    let ptr: *mut T = data.v.get();
	    assert_eq!(unsafe { *ptr }, $value);
	})
    }

    #[test]
    fn atomic_fence_acq_test1() {
	atomic_fence_acq_test!( 68, 500, 500 );
    }
}
