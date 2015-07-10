#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::atomic_cxchg_rel;

    use core::cell::UnsafeCell;

    use std::sync::Arc;

    use std::thread;

    // pub fn atomic_cxchg_rel<T>(dst: *mut T, old: T, src: T) -> T;

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

    macro_rules! atomic_cxchg_rel_test {
	($init:expr, $old:expr, $new:expr, $result:expr) => ({
	    let value: T = $init;
	    let a: A<T> = A::<T>::new(value);

	    let data: Arc<A<T>> = Arc::<A<T>>::new(a);
	    let clone: Arc<A<T>> = data.clone();

	    thread::spawn(move || {
		let dst: *mut T = clone.v.get();
		let old: T = $old;
		let src: T = $new;
		let result: T = unsafe { atomic_cxchg_rel::<T>(dst, old, src) };

		assert_eq!(result, value);
	    });

	    thread::sleep_ms(10);

	    let ptr: *mut T = data.v.get();
	    assert_eq!(unsafe { *ptr }, $result);
	})
    }

    #[test]
    fn atomic_cxchg_rel_test1() {
	atomic_cxchg_rel_test!(  0,  0, 500, 500 );
	atomic_cxchg_rel_test!(  0, 68, 500,   0 );
	atomic_cxchg_rel_test!( 68,  0, 500,  68 );
	atomic_cxchg_rel_test!( 68, 68, 500, 500 );
    }
}
