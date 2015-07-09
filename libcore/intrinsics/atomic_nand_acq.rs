#![feature(core, core_intrinsics)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::intrinsics::atomic_nand_acq;

    use core::cell::UnsafeCell;

    use std::sync::Arc;

    use std::thread;

    // pub fn atomic_nand_acq<T>(dst: *mut T, src: T) -> T;

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

    macro_rules! atomic_nand_acq_test {
	($init:expr, $value:expr, $result:expr) => ({
	    let value: T = $init;
	    let a: A<T> = A::<T>::new(value);

	    let data: Arc<A<T>> = Arc::<A<T>>::new(a);
	    let clone: Arc<A<T>> = data.clone();

	    thread::spawn(move || {
		let dst: *mut T = clone.v.get();
		let src: T = $value;
		let old: T = unsafe { atomic_nand_acq::<T>(dst, src) };

		assert_eq!(old, $init);
	    });

	    thread::sleep_ms(10);

	    let ptr: *mut T = data.v.get();
	    assert_eq!(unsafe { *ptr }, $result);
	})
    }

    #[test]
    fn atomic_nand_acq_test1() {
	atomic_nand_acq_test!( 0xff00, 0x0a0a, !0x0a00 );
    }
}
