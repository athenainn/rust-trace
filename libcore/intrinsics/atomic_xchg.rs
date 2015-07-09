#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::atomic_xchg;

    use core::cell::UnsafeCell;

    use std::sync::Arc;

    use std::thread;

    // pub fn atomic_xchg<T>(dst: *mut T, src: T) -> T;

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

    macro_rules! atomic_xchg_test {
	($init:expr, $value:expr) => ({
	    let value: T = $init;
	    let a: A<T> = A::<T>::new(value);

	    let data: Arc<A<T>> = Arc::<A<T>>::new(a);
	    let clone: Arc<A<T>> = data.clone();

	    thread::spawn(move || {
		let dst: *mut T = clone.v.get();
		let src: T = $value;
		let result: T = unsafe { atomic_xchg::<T>(dst, src) };

		assert_eq!(result, $init);
	    });

	    thread::sleep_ms(10);

	    let ptr: *mut T = data.v.get();
	    assert_eq!(unsafe { *ptr }, $value);
	})
    }

    #[test]
    fn atomic_xchg_test1() {
	atomic_xchg_test!(  68, 500 );
    }
}
