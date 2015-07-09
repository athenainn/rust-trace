#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::atomic_load_relaxed;

    use core::cell::UnsafeCell;

    use std::sync::Arc;

    use std::thread;

    // pub fn atomic_load_relaxed<T>(src: *const T) -> T;

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

    macro_rules! atomic_load_relaxed_test {
	($value:expr) => ({
	    let value: T = $value;
	    let a: A<T> = A::<T>::new(value);

	    let data: Arc<A<T>> = Arc::<A<T>>::new(a);
	    let clone: Arc<A<T>> = data.clone();

	    thread::spawn(move || {
		let src: *mut T = clone.v.get();
		let result: T = unsafe { atomic_load_relaxed::<T>(src) };

		assert_eq!(result, $value);
	    });

	    thread::sleep_ms(10);

	    let ptr: *mut T = data.v.get();
	    assert_eq!(unsafe { atomic_load_relaxed::<T>(ptr) }, $value);
	})
    }

    #[test]
    fn atomic_load_relaxed_test1() {
	atomic_load_relaxed_test!( 68 );
    }
}
